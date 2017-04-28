use std::time::Duration;
use patch::Patch;
use trigger::Trigger;
use effects::note_sequencer::{NoteSequencer};
use effects::sweep_down::{SweepDown};
use effects::control_sequencer::{ControlSequencer};
use midi_devices::{DEFAULT_IN_DEVICE, DEFAULT_OUT_DEVICE};
use utils::{add, concat, repeated};


const CUTOFF: u8 = 74;

pub fn create_test_song() -> Patch {
    let chorus_notes = repeated(&concat(vec![
        repeated(&[45, 57], 4),
        repeated(&[48, 60], 4),
        repeated(&[43, 55], 4),
        repeated(&[38, 50], 4)
    ]), 6);

    let wild_notes = repeated(&[45, 47, 53, 57, 60, 67, 60, 57, 53, 47], 50);

    let speed = 220;

    let note_seq = |notes: Vec<u8>| {
        NoteSequencer::new(DEFAULT_OUT_DEVICE, notes, Duration::from_millis(speed), 0x7f)
    };

    Patch::new("Test", vec![
        (Box::new(Trigger::new(DEFAULT_IN_DEVICE, 43)), Box::new(note_seq(chorus_notes.clone()))),
        (Box::new(Trigger::new(DEFAULT_IN_DEVICE, 43)), Box::new(NoteSequencer::new(DEFAULT_OUT_DEVICE, add(wild_notes.clone(), 24), Duration::from_millis(speed / 2), 0x7f))),
        (Box::new(Trigger::new(DEFAULT_IN_DEVICE, 45)), Box::new(note_seq(chorus_notes))),
        (Box::new(Trigger::new(DEFAULT_IN_DEVICE, 36)), Box::new(note_seq(wild_notes))),
        (Box::new(Trigger::new(DEFAULT_IN_DEVICE, 52)), Box::new(note_seq(vec![]))),
        (Box::new(Trigger::new(DEFAULT_IN_DEVICE, 50)), Box::new(SweepDown::new(DEFAULT_OUT_DEVICE, 30, CUTOFF))),
        (
            Box::new(Trigger::new(DEFAULT_IN_DEVICE, 48)),
            Box::new(ControlSequencer::new(DEFAULT_OUT_DEVICE, CUTOFF, vec![30, 100, 30, 100], 30, Duration::from_millis(500)))
        )
    ], 0)
}
