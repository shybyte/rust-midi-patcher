extern crate portmidi as pm;

use pm::{MidiMessage, DeviceInfo};

use trigger::Trigger;
use effect::{Effect};
use absolute_sleep::AbsoluteSleep;

pub struct Patch {
    effects: Vec<(Box<Trigger>, Box<Effect>)>,
}

impl Patch {
    pub fn new(effects: Vec<(Box<Trigger>, Box<Effect>)>) -> Patch {

        Patch {
            effects: effects
        }
    }

    pub fn on_midi_event(&mut self, device: &DeviceInfo, midi_message: MidiMessage) {
        println!("Patch.on_midi_event {:?}  {:?}", device, midi_message);
        let triggered_effect_indices: Vec<_> = (0..self.effects.len()).filter(|&i| self.effects[i].0.is_triggered(device, midi_message)).collect();
        if !triggered_effect_indices.is_empty() {
            let absolute_sleep = AbsoluteSleep::new();
            self.stop_running_effects();
            for triggered_index in triggered_effect_indices {
                self.effects[triggered_index].1.start(midi_message, absolute_sleep);
            }
        }
    }

    fn stop_running_effects(&mut self) {
        for &mut (_, ref mut eff) in &mut self.effects {
            if eff.is_running() {
                eff.stop();
            }
        }
    }
}
