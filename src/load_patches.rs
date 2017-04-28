use std::time::Duration;
use std::fs;
use std::path::Path;

use risp::eval_risp_script;
use risp::types::{RispType, RispError, error};
use risp::core::create_core_environment;
use risp::convert::flatten_into;
use risp::types::RispType::*;

use patch::Patch;
use utils::read_file;
use trigger::Trigger;
use effects::effect::Effect;
use effects::note_sequencer::{NoteSequencer};
use effects::sweep_down::{SweepDown};
use effects::control_sequencer::{ControlSequencer};

use microkorg::CUTOFF;

use config::Config;

use songs::test::create_test_song;
use songs::polly::create_polly;

pub fn load_patches(config: &Config) -> Vec<Patch> {
    let mut patches = vec![create_test_song(), create_polly()];

    let paths = fs::read_dir("patches").unwrap();

    for path in paths {
        let patch_path = path.unwrap().path();
        println!("Loading Patch {:?}", patch_path.display());
        match load_patch(&patch_path, config) {
            Ok(amazon) => {
                println!("Loaded Patch {:?}", patch_path.display());
                patches.push(amazon);
            }
            Err(error) => {
                println!("Error while loading: {:?}", error);
            }
        }
    }

    patches
}

fn load_patch(file_name: &Path, config: &Config) -> Result<Patch, RispError> {
    let risp_code = read_file(file_name).map_err(|_| error(format!("Can't read file {:?}", file_name.display())))?;

    let mut env = create_core_environment();
    env.set("CUTOFF", Int(CUTOFF as i64));

    eval_risp_script(&risp_code, &mut env)
        .and_then(|patch_risp| {
            let program = patch_risp.get("program")?.unwrap_or(0);
            let name: String = patch_risp.get("name")?.ok_or_else(|| error("Missing Name"))?;
            let time_per_note = patch_risp.get("time_per_note")?.unwrap_or(200);
            let effects_risp: Vec<RispType> = patch_risp.get("effects")?.ok_or_else(|| error("Missing effects"))?;
            let effects: Result<_, _> = effects_risp.into_iter().map(|e| to_trigger_effect_pair(&e, time_per_note, config)).collect();
            Ok(Patch::new(name, effects?, program as u8))
        })
}

// {:trigger 45 :noteSequencer {:notes [38 50 38 50 chorus_notes] :beat_offset 4}
fn to_trigger_effect_pair(input: &RispType, default_time_per_note: i64, config: &Config) -> Result<(Box<Trigger>, Box<Effect>), RispError> {
    let trigger_note = input.get("trigger")?.unwrap_or(0);
    let trigger = Box::new(Trigger::new(&config.default_in_device, trigger_note as u8));

    let note_sequencer_risp_option: Option<RispType> = input.get("noteSequencer")?;
    if let Some(note_sequencer_risp) = note_sequencer_risp_option {
        let beat_offset = note_sequencer_risp.get("beat_offset")?.unwrap_or(0) as usize;
        let time_per_note = note_sequencer_risp.get("time_per_note")?.unwrap_or(default_time_per_note) as u64;

        let notes_risp = note_sequencer_risp.get("notes")?.ok_or_else(|| error("Missing notes"))?;
        let notes = flatten_into(notes_risp)?.iter().map(|&x: &i64| x as u8).collect();
        let effect = Box::new(NoteSequencer::new_with_beat_offset(&config.default_out_device, notes, Duration::from_millis(time_per_note), 0x7f, beat_offset));
        return Ok((trigger, effect));
    }

    let sweep_down_risp_option: Option<RispType> = input.get("sweepDown")?;
    if let Some(sweep_down) = sweep_down_risp_option {
        let min_value = sweep_down.get("min_value")?.unwrap_or(30) as u8;
        let control_index: i64 = sweep_down.get("control_index")?.ok_or_else(|| error("Missing control_index"))?;
        let effect = Box::new(SweepDown::new(&config.default_out_device, min_value, control_index as u8));
        return Ok((trigger, effect));
    }

    let control_sequencer_risp_option: Option<RispType> = input.get("controlSequencer")?;
    if let Some(control_sequencer_risp) = control_sequencer_risp_option {
        let control_index: i64 = control_sequencer_risp.get("control_index")?.ok_or_else(|| error("Missing control_index"))?;
        let stop_value: i64 = control_sequencer_risp.get("stop_value")?.ok_or_else(|| error("Missing stop_value"))?;
        let time_per_note = control_sequencer_risp.get("time_per_note")?.unwrap_or(default_time_per_note) as u64;

        let values_risp = control_sequencer_risp.get("values")?.ok_or_else(|| error("Missing notes"))?;
        let values = flatten_into(values_risp)?.iter().map(|&x: &i64| x as u8).collect();
        let effect = Box::new(ControlSequencer::new(&config.default_out_device, control_index as u8, values,
                                                    stop_value as u8, Duration::from_millis(time_per_note)));
        return Ok((trigger, effect));
    }

    Err(error("Missing effect"))
}

