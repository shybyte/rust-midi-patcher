use std::time::Duration;
use patch::Patch;
use trigger::Trigger;
use effects::note_sequencer::NoteSequencer;
use effects::harmony_drum::HarmonyDrum;
use utils::{concat, repeated};
use config::Config;
use midi_notes::*;
use midi_devices::*;


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
        //        (Box::new(Trigger::new(&config.default_in_device, 43)), Box::new(note_seq(chorus_notes.clone()))),
        //        (Box::new(Trigger::new(SAMPLE_PAD, 45)), Box::new(note_seq(vec![]))),
        (
            Box::new(Trigger::new(SAMPLE_PAD, 45)),
            Box::new(HarmonyDrum::new(USB_MIDI_ADAPTER, USB_MIDI_ADAPTER, (C2, D3), vec![0]))
        ),
        (
            Box::new(Trigger::new(SAMPLE_PAD, 51)),
            Box::new(HarmonyDrum::new(USB_MIDI_ADAPTER, USB_MIDI_ADAPTER, (C2, D3), vec![0, 7, 12]))
        ),
    ], 0)
}
