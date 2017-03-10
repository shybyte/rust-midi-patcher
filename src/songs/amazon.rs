use pm::{PortMidi};

use std::time::Duration;
use patch::Patch;
use std::sync::{Arc, Mutex};
use trigger::Trigger;
use effect::{NoteSequencer};
use midi_devices::{DEFAULT_IN_DEVICE, DEFAULT_OUT_DEVICE};
use utils::{add, concat, repeated};
const BUF_LEN: usize = 1024;


pub fn create_amazon(context: &PortMidi) -> Patch {
    let first_out_device = context.devices().unwrap().into_iter()
        .find(|dev| dev.is_output() && dev.name().contains(DEFAULT_OUT_DEVICE))
        .unwrap();
    let output_port = context.output_port(first_out_device, BUF_LEN).unwrap();
    let output_port_arc = Arc::new(Mutex::new(output_port));

    let chorus_notes = repeated(&concat(vec![
        repeated(&[45, 57], 4),
        repeated(&[48, 60], 4),
        repeated(&[43, 55], 4),
        repeated(&[38, 50], 4)
    ]), 6);

    let wild_notes = repeated(&[45, 47, 53, 57, 60, 67, 60, 57, 53, 47], 50);

    let speed = 220;

    let note_seq = |notes: Vec<u8>| {
        NoteSequencer::new(output_port_arc.clone(), notes, Duration::from_millis(speed), 0x7f)
    };

    Patch::new(vec![
        (Box::new(Trigger::new(DEFAULT_IN_DEVICE, 43)), Box::new(note_seq(chorus_notes.clone()))),
        (Box::new(Trigger::new(DEFAULT_IN_DEVICE, 43)), Box::new(NoteSequencer::new(output_port_arc.clone(), add(wild_notes.clone(), 24), Duration::from_millis(speed / 2), 0x7f))),
        (Box::new(Trigger::new(DEFAULT_IN_DEVICE, 45)), Box::new(note_seq(chorus_notes))),
        (Box::new(Trigger::new(DEFAULT_IN_DEVICE, 36)), Box::new(note_seq(wild_notes))),
        (Box::new(Trigger::new(DEFAULT_IN_DEVICE, 52)), Box::new(note_seq(vec![])))
    ])
}
