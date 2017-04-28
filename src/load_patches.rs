use std::time::Duration;
use std::fs;
use std::path::Path;

use risp::eval_risp_script;
use risp::types::{RispType, RispError, error};
use risp::core::create_core_environment;
use risp::convert::flatten_into;

use patch::Patch;
use utils::read_file;
use trigger::Trigger;
use effects::effect::Effect;
use effects::note_sequencer::{NoteSequencer};

use midi_devices::{DEFAULT_IN_DEVICE, DEFAULT_OUT_DEVICE};

use songs::test::create_test_song;
use songs::polly::create_polly;

pub fn load_patches() -> Vec<Patch> {
    let mut patches = vec![create_test_song(), create_polly()];

    let paths = fs::read_dir("patches").unwrap();

    for path in paths {
        let patch_path = path.unwrap().path();
        println!("Loading Patch {:?}", patch_path.display());
        match load_patch(&patch_path) {
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

fn load_patch(file_name: &Path) -> Result<Patch, RispError> {
    let risp_code = read_file(file_name).map_err(|_| error(format!("Can't read file {:?}", file_name.display())))?;

    let mut env = create_core_environment();
    eval_risp_script(&risp_code, &mut env)
        .and_then(|patch_risp| {
            let program = patch_risp.get("program")?.unwrap_or(0);
            let name: String = patch_risp.get("name")?.ok_or_else(|| error("Missing Name"))?;
            let time_per_note = patch_risp.get("time_per_note")?.unwrap_or(200);
            let effects_risp: Vec<RispType> = patch_risp.get("effects")?.ok_or_else(|| error("Missing effects"))?;
            let effects: Result<_, _> = effects_risp.into_iter().map(|e| to_trigger_effect_pair(&e, time_per_note)).collect();
            Ok(Patch::new(name, effects?, program as u8))
        })
}

// {:trigger 45 :noteSequencer {:notes [38 50 38 50 chorus_notes] :beat_offset 4}
fn to_trigger_effect_pair(input: &RispType, default_time_per_note: i64) -> Result<(Box<Trigger>, Box<Effect>), RispError> {
    let trigger_note = input.get("trigger")?.unwrap_or(0);
    let trigger = Box::new(Trigger::new(DEFAULT_IN_DEVICE, trigger_note as u8));

    let note_sequencer_risp: RispType = input.get("noteSequencer")?.ok_or_else(|| error("Missing noteSequencer"))?;
    let beat_offset = note_sequencer_risp.get("beat_offset")?.unwrap_or(0) as usize;
    let time_per_note = note_sequencer_risp.get("time_per_note")?.unwrap_or(default_time_per_note) as u64;

    let notes_risp = note_sequencer_risp.get("notes")?.ok_or_else(|| error("Missing notes"))?;
    let notes = flatten_into(notes_risp)?.iter().map(|&x: &i64| x as u8).collect();
    let effect = Box::new(NoteSequencer::new_with_beat_offset(DEFAULT_OUT_DEVICE, notes, Duration::from_millis(time_per_note), 0x7f, beat_offset));

    Ok((trigger, effect))
}

