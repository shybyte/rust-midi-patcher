extern crate portmidi as pm;


use pm::{MidiMessage, DeviceInfo, PortMidi};

use std::sync::{Arc, Mutex};
use trigger::Trigger;
use effect::{Effect, NoteSequencer};
use midi_devices::{VMPK};
use std::time::Duration;
use std::iter;

const BUF_LEN: usize = 1024;


pub struct Patch {
    effects: Vec<(Box<Trigger>, Box<Effect>)>,
    running_effect: Option<usize>
}

impl Patch {
    pub fn new(context: &PortMidi) -> Patch {
        let first_out_device = context.devices().unwrap().into_iter()
            .find(|dev| dev.is_output())
            .unwrap();
        let output_port = context.output_port(first_out_device, BUF_LEN).unwrap();
        let output_port_arc = Arc::new(Mutex::new(output_port));

        let trigger = Trigger::new(VMPK, 45);
        let eff = NoteSequencer::new(
            output_port_arc.clone(),
            repeated(&concat(vec![
                repeated(&[45, 57], 4),
                repeated(&[48, 60], 4),
                repeated(&[43, 55], 4),
                repeated(&[38, 50], 4)
            ]), 6),
            Duration::from_millis(200)
        );

        let random = NoteSequencer::new(
            output_port_arc.clone(),
            repeated(&[45, 47, 53, 57, 60, 67, 60, 57, 53, 47], 50),
            Duration::from_millis(200)
        );

        let nada = NoteSequencer::new(
            output_port_arc.clone(),
            vec![],
            Duration::from_millis(200)
        );
        Patch {
            effects: vec![
                (Box::new(trigger), Box::new(eff)),
                (Box::new(Trigger::new(VMPK, 36)), Box::new(random)),
                (Box::new(Trigger::new(VMPK, 52)), Box::new(nada))
            ],
            running_effect: None
        }
    }

    pub fn on_midi_event(&mut self, device: &DeviceInfo, midi_message: MidiMessage) {
        println!("Patch.on_midi_event {:?}  {:?}", device, midi_message);
        let triggered_effect_option = self.effects.iter()
            .position(|&(ref trigger, _)| trigger.is_triggered(device, midi_message));
        if let Some(triggered_index) = triggered_effect_option {
            if let Some(running_effect) = self.running_effect {
                self.effects[running_effect].1.stop();
            }
            if let Some(&mut (_, ref mut effect)) = self.effects.get_mut(triggered_index) {
                println!("start effect");
                effect.start(midi_message);
                self.running_effect = triggered_effect_option;
            }
        }
        //let triggered = self.effects.iter_mut()
        //    .find(|&&mut (ref trigger, _)| trigger.is_triggered(device, midi_message));
    }
}

pub fn repeated<T: Clone>(pattern: &[T], times: usize) -> Vec<T> {
    concat(iter::repeat(pattern.iter().cloned().collect()).take(times).collect())
}

pub fn concat<T: Clone>(input: Vec<Vec<T>>) -> Vec<T> {
    input.iter().cloned().flat_map(|x| x).collect()
}

