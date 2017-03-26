use std::time::Duration;
use patch::Patch;
use trigger::Trigger;
use effects::note_sequencer::{NoteSequencer};
use midi_devices::{DEFAULT_IN_DEVICE, DEFAULT_OUT_DEVICE};
use utils::{concat, repeated};

pub fn create_kirschblueten() -> Patch {
    let chorus_notes = repeated(&concat(vec![
        repeated(&[50, 65], 4),
        repeated(&[48, 67], 4),
        repeated(&[46, 62], 4),
        repeated(&[46, 62], 4),
        //
        repeated(&[50, 65], 4),
        repeated(&[48, 67], 4),
        repeated(&[46, 62], 4),
        repeated(&[46, 62], 4),
        //
        repeated(&[50, 65], 4),
        repeated(&[45, 64], 4),
        repeated(&[41, 67], 4),
        repeated(&[43, 69], 4),
        //
        repeated(&[50, 65], 4),
        repeated(&[48, 67], 4),
        repeated(&[46, 62], 4),
        repeated(&[46, 62], 4)
    ]), 1);

    let speed = 210;

    let note_seq = |notes: Vec<u8>| {
        NoteSequencer::new(DEFAULT_OUT_DEVICE, notes, Duration::from_millis(speed), 0x7f)
    };

    Patch::new(vec![
        (Box::new(Trigger::new(DEFAULT_IN_DEVICE, 38)), Box::new(note_seq(chorus_notes.clone()))),
        (Box::new(Trigger::new(DEFAULT_IN_DEVICE, 36)), Box::new(note_seq(vec![])))
    ], 53)
}
