use std::time::Duration;
use patch::Patch;
use trigger::Trigger;
use effects::note_sequencer::NoteSequencer;
use effects::harmony_drum::HarmonyDrum;
use utils::{concat, repeated};
use config::Config;
use midi_notes::*;


pub fn create_harmony_drum_test_song(config: &Config) -> Patch {
    let chorus_notes = repeated(&concat(vec![
        repeated(&[45, 57], 4),
        repeated(&[48, 60], 4),
        repeated(&[43, 55], 4),
        repeated(&[38, 50], 4)
    ]), 6);


    let speed = 220;

    let note_seq = |notes: Vec<u8>| {
        NoteSequencer::new(&config.default_out_device, notes, Duration::from_millis(speed), 0x7f)
    };

    Patch::new("HarmonyTest", vec![
        (Box::new(Trigger::new(&config.default_in_device, 43)), Box::new(note_seq(chorus_notes.clone()))),
        (Box::new(Trigger::new(&config.default_in_device, 52)), Box::new(note_seq(vec![]))),
        (
            Box::new(Trigger::new(&config.default_in_device, C5)),
            Box::new(HarmonyDrum::new(&config.default_in_device, &config.default_out_device, (C3, C4)))
        ),
    ], 0)
}
