extern crate portmidi as pm;

use pm::{MidiMessage, DeviceInfo, OutputPort};

use trigger::Trigger;
use effects::effect::{Effect, MonoGroup};
use absolute_sleep::AbsoluteSleep;
use std::sync::{Arc, Mutex};
use chan::{Sender};
use view::main_view::ToViewEvents;

pub struct Patch {
    name: String,
    effects: Vec<(Box<Trigger>, Box<Effect>)>,
    program: u8,
}

impl Patch {
    pub fn new<S: Into<String>>(name: S, effects: Vec<(Box<Trigger>, Box<Effect>)>, program: u8) -> Patch {
        Patch {
            name: name.into(),
            effects: effects,
            program: program
        }
    }

    pub fn on_midi_event(&mut self, output_ports: &[Arc<Mutex<OutputPort>>], device: &DeviceInfo, midi_message: MidiMessage, to_view_tx: &Sender<ToViewEvents>) {
        // println!("Patch.on_midi_event {:?}  {:?}", device, midi_message);
        let triggered_effect_indices: Vec<usize> = (0..self.effects.len()).filter(|&i| self.effects[i].0.is_triggered(device, midi_message)).collect();
        if !triggered_effect_indices.is_empty() {
            let triggered_mono_groups: Vec<MonoGroup> = triggered_effect_indices.iter().map(|&i| self.effects[i].1.mono_group()).collect();
            for triggered_mono_group in triggered_mono_groups {
                self.stop_running_effects_in_mono_group(triggered_mono_group);
            }
            let absolute_sleep = AbsoluteSleep::new();
            for triggered_index in triggered_effect_indices {
                self.effects[triggered_index].1.start(output_ports, midi_message, absolute_sleep, to_view_tx);
            }
        }
    }

    fn stop_running_effects_in_mono_group(&mut self, mono_group: MonoGroup) {
        for &mut (_, ref mut eff) in &mut self.effects {
            if eff.is_running() && eff.mono_group() == mono_group {
                eff.stop();
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

    pub fn name(&self) -> &str {
        &self.name[..]
    }
}
