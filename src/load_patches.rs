use std::fs;
use std::path::Path;
use std::time::{Duration, Instant};

use risp::eval_risp_script;
use risp::types::{RispType, RispError, error};
use risp::core::create_core_environment;
use risp::convert::flatten_into;
use risp::types::RispType::*;

use crate::patch::Patch;
use crate::utils::read_file;
use crate::trigger::Trigger;
use crate::effects::effect::Effect;
use crate::effects::note_sequencer::NoteSequencer;
use crate::effects::sweep_down::SweepDown;
use crate::effects::control_sequencer::ControlSequencer;
use crate::virtual_midi::MidiLightPatch;


use crate::microkorg::*;

use crate::config::Config;

use crate::songs::test::create_test_song;
use crate::songs::harmony_drum_test::create_harmony_drum_test_song;
use crate::songs::endstation_paradies::*;
use crate::songs::endstation::liebt_uns::liebt_uns;
use crate::songs::endstation::diktator::diktator;

pub fn load_patches(config: &Config) -> Vec<Patch> {
    let mut patches = vec![
        create_test_song(),
        create_harmony_drum_test_song(config),
        wahrheit(config),
        enddzeit(config),
        liebeslieder(config),
        sicherheitskopie(config),
        young(config),
        diktator(config),
        liebt_uns(config),
        system(config),
    ];

    let paths = fs::read_dir("patches").unwrap();

    for path in paths {
        let patch_path = path.unwrap().path();
        println!("Loading Patch {:?}", patch_path.display());
        let loading_start_time = Instant::now();
        match load_patch(&patch_path, config) {
            Ok(amazon) => {
                let load_time: Duration = Instant::now() - loading_start_time;
                println!("Loaded Patch {:?} in {:?}", patch_path.display(), load_time.subsec_nanos() / 1_000_000);
                patches.push(amazon);
            }
            Err(error) => {
                println!("Error while loading: {:?}", error);
            }
        }
    }

    patches
}

pub fn load_patch(file_name: &Path, config: &Config) -> Result<Patch, RispError> {
    let start_time = Instant::now();
    let risp_code = read_file(file_name).map_err(|_| error(format!("Can't read file {:?}", file_name.display())))?;
    println!("Read Patch File in {:?} ms", (Instant::now() - start_time).subsec_nanos() / 1_000_000);

    let mut env = create_core_environment();
    env.set("CUTOFF", Int(i64::from(CUTOFF)));
    env.set("OSC2_SEMITONE", Int(i64::from(OSC2_SEMITONE)));

    eval_risp_script(&risp_code, &mut env)
        .and_then(|patch_risp| {
            println!("Evaluated Patch File in {:?} ms", (Instant::now() - start_time).subsec_nanos() / 1_000_000);
            let program = patch_risp.get("program")?.unwrap_or(0);
            let name: String = patch_risp.get("name")?.ok_or_else(|| error("Missing Name"))?;
            let time_per_note = patch_risp.get("time_per_note")?.unwrap_or(200);
            let effects_risp: Vec<RispType> = patch_risp.get("effects")?.ok_or_else(|| error("Missing effects"))?;
            let effects: Result<_, _> = effects_risp.into_iter().map(|e| to_trigger_effect_pair(&e, time_per_note, config)).collect();
            let lights_patch: Option<MidiLightPatch> = patch_risp.get("lights")?.map(|l| to_lights(&l));
            Ok(Patch::new(name, effects?, program as u8, lights_patch))
        })
}

pub fn to_lights(lights_risp: &RispType) -> MidiLightPatch {
    let mut midi_light_patch = MidiLightPatch::default();
    midi_light_patch.stream = lights_risp.get("stream").unwrap().unwrap_or(false);
    midi_light_patch.blink = lights_risp.get("blink").unwrap().unwrap_or(false);
    midi_light_patch.flash = lights_risp.get("flash").unwrap().unwrap_or(false);
    midi_light_patch.ripples = lights_risp.get("ripples").unwrap().unwrap_or(false);
    midi_light_patch.push = lights_risp.get("push").unwrap().unwrap_or(false);
    midi_light_patch.fish = lights_risp.get("fish").unwrap().unwrap_or(false);
    midi_light_patch.river = lights_risp.get("river").unwrap().unwrap_or(false);
    midi_light_patch.stream_center = lights_risp.get("stream_center").unwrap().unwrap_or(false);
    let max_note: i64 = lights_risp.get("max_note").unwrap().unwrap_or(128);
    midi_light_patch.max_note = max_note as u8;
    midi_light_patch
}

// {:trigger 45 :noteSequencer {:notes [38 50 38 50 chorus_notes]}
fn to_trigger_effect_pair(input: &RispType, default_time_per_note: i64, config: &Config) -> Result<(Box<Trigger>, Box<Effect>), RispError> {
    let trigger_risp: RispType = input.get("trigger")?.ok_or_else(|| error("Missing trigger"))?;
    let trigger = match trigger_risp {
        Int(trigger_note) => Box::new(Trigger::new(&config.default_in_device, trigger_note as u8)),
        Vector(ref args) if args.len() == 2 => {
            let device: Result<String, _> = args[0].clone().into();
            let note: Result<i64, _> = args[1].clone().into();
            Box::new(Trigger::new(&device?, note? as u8))
        }
        _ => return Err(error("Strange trigger"))
    };

    let note_sequencer_risp_option: Option<RispType> = input.get("noteSequencer")?;
    if let Some(note_sequencer_risp) = note_sequencer_risp_option {
        let time_per_note = note_sequencer_risp.get("time_per_note")?.unwrap_or(default_time_per_note) as u64;

        let notes_risp = note_sequencer_risp.get("notes")?.ok_or_else(|| error("Missing notes"))?;
        let notes = flatten_into(notes_risp)?.iter().map(|&x: &i64| x as u8).collect();
        let effect = Box::new(NoteSequencer::new(&config.default_out_device, notes, Duration::from_millis(time_per_note), 0x7f));
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

