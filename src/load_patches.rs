use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::time::Duration;


use risp::eval_risp_script;
use risp::types::RispType;
use risp::core::create_core_environment;

use patch::Patch;
use trigger::Trigger;
use effects::effect::Effect;
use effects::note_sequencer::{NoteSequencer};

use midi_devices::{DEFAULT_IN_DEVICE, DEFAULT_OUT_DEVICE};

use songs::kirschblueten::create_kirschblueten;
use songs::test::create_test_song;
use songs::polly::create_polly;

type LoadError = String;

pub fn load_patches() -> Vec<Patch> {
    let mut patches = vec![create_test_song(), create_kirschblueten(), create_polly()];

    match load_patch("src/songs/amazon.risp") {
        Ok(amazon) => {
            println!("Loaded amazon");
            patches.push(amazon);
        }
        Err(error) => {
            println!("Error while loading amazon = {:?}", error);
        }
    }

    patches
}

fn load_patch(file_name: &str) -> Result<Patch, LoadError> {
    let mut file = File::open(file_name).unwrap();
    let mut risp_code = String::new();
    file.read_to_string(&mut risp_code).map_err(|_| format!("Can't read file {:?}", file_name))?;

    let mut env = create_core_environment();
    eval_risp_script(&risp_code, &mut env)
        .map_err(|err| format!("Risp Error {:?}", err))
        .and_then(|patch_risp| {
            let patch_map = to_map(patch_risp)?;
            let program = get_or(&patch_map, "program", to_int, 0)?;
            let time_per_note = get_or(&patch_map, "time_per_note", to_int, 200)?;
            let effects_risp = get_or(&patch_map, "effects", to_vec, vec![])?;
            let effects: Result<_, _> = effects_risp.iter().cloned().map(|e| to_trigger_effect_pair(e, time_per_note)).collect();
            Ok(Patch::new(effects?, program as u8))
        })
}

// {:trigger 45 :noteSequencer {:notes [38 50 38 50 chorus_notes] :beat_offset 4}
fn to_trigger_effect_pair(input: RispType, default_time_per_note: i64) -> Result<(Box<Trigger>, Box<Effect>), LoadError> {
    let input_map = to_map(input)?;

    let trigger_note = get_or(&input_map, "trigger", to_int, 0)? as u8;
    let trigger = Box::new(Trigger::new(DEFAULT_IN_DEVICE, trigger_note));

    let note_sequencer_risp = input_map.get("noteSequencer").ok_or_else(|| "Missing noteSequencer")?;
    let note_sequencer_map = to_map(note_sequencer_risp.clone())?;
    let beat_offset = get_or(&note_sequencer_map, "beat_offset", to_int, 0)? as usize;
    let time_per_note = get_or(&note_sequencer_map, "time_per_note", to_int, default_time_per_note)? as u64;
    let notes_risp = get_or(&note_sequencer_map, "notes", to_vec, vec![])?;
    let notes = flatten_vec(notes_risp).iter().cloned().map(to_u8).collect::<Result<Vec<_>, _>>()?;
    println!("notes = {:?}", notes);
    let effect = Box::new(NoteSequencer::new_with_beat_offset(DEFAULT_OUT_DEVICE, notes, Duration::from_millis(time_per_note), 0x7f, beat_offset));

    Ok((trigger, effect))
}

fn get_or<T, F>(map: &HashMap<String, RispType>, key: &str, f: F, default: T) -> Result<T, LoadError>
    where F: FnOnce(RispType) -> Result<T, LoadError> {
    map.get(key).cloned().map_or(Ok(default), f)
}

fn to_map(risp_map: RispType) -> Result<HashMap<String, RispType>, LoadError> {
    match risp_map {
        RispType::Map(map) => Ok(map),
        _ => Err(format!("Expected Map but got {:?}", risp_map))
    }
}

#[allow(needless_pass_by_value)]
fn to_int(risp_int: RispType) -> Result<i64, LoadError> {
    match risp_int {
        RispType::Int(int) => Ok(int),
        _ => Err(format!("Expected Int but got {:?}", risp_int)),
    }
}

#[allow(needless_pass_by_value)]
fn to_u8(risp_int: RispType) -> Result<u8, LoadError> {
    match risp_int {
        RispType::Int(int) => Ok(int as u8),
        _ => Err(format!("Expected U8 but got {:?}", risp_int)),
    }
}

fn to_vec(risp_vec: RispType) -> Result<Vec<RispType>, LoadError> {
    match risp_vec {
        RispType::Vector(vector) => Ok(vector),
        _ => Err(format!("Expected Vector but got {:?}", risp_vec)),
    }
}

fn flatten_vec(risp_vec: Vec<RispType>) -> Vec<RispType> {
    let mut result = vec![];
    for el in risp_vec {
        if let RispType::Vector(vector) = el {
            for child_el in flatten_vec(vector) {
                result.push(child_el)
            }
        } else {
            result.push(el);
        }
    }
    result
}