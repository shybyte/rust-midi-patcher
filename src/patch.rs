extern crate portmidi as pm;

use pm::{MidiMessage, DeviceInfo, OutputPort};

use trigger::Trigger;
use effects::effect::{Effect};
use absolute_sleep::AbsoluteSleep;
use std::sync::{Arc, Mutex};

pub struct Patch {
    effects: Vec<(Box<Trigger>, Box<Effect>)>,
    program: u8,
}

impl Patch {
    pub fn new(effects: Vec<(Box<Trigger>, Box<Effect>)>, program: u8) -> Patch {
        Patch {
            effects: effects,
            program: program
        }
    }

    pub fn on_midi_event(&mut self, output_ports: &[Arc<Mutex<OutputPort>>], device: &DeviceInfo, midi_message: MidiMessage) {
        println!("Patch.on_midi_event {:?}  {:?}", device, midi_message);
        let triggered_effect_indices: Vec<_> = (0..self.effects.len()).filter(|&i| self.effects[i].0.is_triggered(device, midi_message)).collect();
        if !triggered_effect_indices.is_empty() {
            let absolute_sleep = AbsoluteSleep::new();
            self.stop_running_effects();
            for triggered_index in triggered_effect_indices {
                self.effects[triggered_index].1.start(output_ports, midi_message, absolute_sleep);
            }
        }
    }

    pub fn stop_running_effects(&mut self) {
        for &mut (_, ref mut eff) in &mut self.effects {
            if eff.is_running() {
                eff.stop();
            }
        }
    }

    pub fn program(&self) -> u8 {
        self.program
    }
}
