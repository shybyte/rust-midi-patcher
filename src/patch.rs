extern crate portmidi as pm;

use std::ops::Add;
use pm::{MidiMessage, DeviceInfo, PortMidi};

use std::sync::{Arc, Mutex};
use trigger::Trigger;
use effect::{Effect, NoteSequencer};
use midi_devices::{DEFAULT_IN_DEVICE, DEFAULT_OUT_DEVICE};
use std::time::Duration;
use std::iter;

const BUF_LEN: usize = 1024;


pub struct Patch {
    effects: Vec<(Box<Trigger>, Box<Effect>)>,
}

impl Patch {
    pub fn new(context: &PortMidi) -> Patch {
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

        let note_seq = |notes: Vec<u8>| {
            NoteSequencer::new(output_port_arc.clone(), notes, Duration::from_millis(200), 0x7f)
        };

        Patch {
            effects: vec![
                (Box::new(Trigger::new(DEFAULT_IN_DEVICE, 43)), Box::new(note_seq(chorus_notes.clone()))),
                (Box::new(Trigger::new(DEFAULT_IN_DEVICE, 43)), Box::new(NoteSequencer::new(output_port_arc.clone(), add(wild_notes.clone(), 24), Duration::from_millis(100), 0x7f))),
                (Box::new(Trigger::new(DEFAULT_IN_DEVICE, 45)), Box::new(note_seq(chorus_notes))),
                (Box::new(Trigger::new(DEFAULT_IN_DEVICE, 36)), Box::new(note_seq(wild_notes))),
                (Box::new(Trigger::new(DEFAULT_IN_DEVICE, 52)), Box::new(note_seq(vec![])))
            ]
        }
    }

    pub fn on_midi_event(&mut self, device: &DeviceInfo, midi_message: MidiMessage) {
        println!("Patch.on_midi_event {:?}  {:?}", device, midi_message);
        let triggered_effect_indices: Vec<_> = (0..self.effects.len()).filter(|&i| self.effects[i].0.is_triggered(device, midi_message)).collect();
        if triggered_effect_indices.len() > 0 {
            self.stop_running_effects();
            for triggered_index in triggered_effect_indices {
                self.effects.get_mut(triggered_index).unwrap().1.start(midi_message);
            }
        }
    }

    fn stop_running_effects(&mut self) {
        for &mut (_, ref mut eff) in self.effects.iter_mut() {
            if eff.is_running() {
                eff.stop();
            }
        }
    }
}

pub fn repeated<T: Clone>(pattern: &[T], times: usize) -> Vec<T> {
    concat(iter::repeat(pattern.iter().cloned().collect()).take(times).collect())
}

pub fn add<T>(mut xs: Vec<T>, y: T) -> Vec<T>
    where T: Copy + Add<T, Output=T> {
    for x in &mut xs {
        *x = *x + y;
    }
    xs
}

pub fn concat<T: Clone>(input: Vec<Vec<T>>) -> Vec<T> {
    input.iter().cloned().flat_map(|x| x).collect()
}

