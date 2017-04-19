use std::time::Duration;
use patch::Patch;
use trigger::Trigger;
use effects::note_sequencer::{NoteSequencer};
use midi_devices::{DEFAULT_IN_DEVICE, DEFAULT_OUT_DEVICE};
use utils::{concat,concatenated, repeated};

#[allow(dead_code)]
pub fn create_amazon() -> Patch {
    let chorus_notes = repeated(&concat(vec![
        repeated(&[45, 57], 4),
        repeated(&[48, 60], 4),
        repeated(&[43, 55], 4),
        repeated(&[38, 50], 4)
    ]), 6);

    let chorus_notes_with_intro = concatenated(&[&[38, 50, 38, 50], &chorus_notes]);

    let wild_notes = repeated(&[45, 47, 53, 57, 60, 67, 60, 57, 53, 47], 50);

    let speed = 220;

    let note_seq = |notes: Vec<u8>| {
        NoteSequencer::new(DEFAULT_OUT_DEVICE, notes, Duration::from_millis(speed), 0x7f)
    };

    Patch::new(vec![
        (Box::new(Trigger::new(DEFAULT_IN_DEVICE, 43)), Box::new(note_seq(chorus_notes.clone()))),
        (Box::new(Trigger::new(DEFAULT_IN_DEVICE, 45)), Box::new(NoteSequencer::new_with_beat_offset(DEFAULT_OUT_DEVICE, chorus_notes_with_intro, Duration::from_millis(speed), 0x7f, 4))),
        (Box::new(Trigger::new(DEFAULT_IN_DEVICE, 36)), Box::new(note_seq(wild_notes))),
        (Box::new(Trigger::new(DEFAULT_IN_DEVICE, 52)), Box::new(note_seq(vec![])))
    ], 42)
}
