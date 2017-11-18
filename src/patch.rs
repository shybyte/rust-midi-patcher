extern crate portmidi as pm;

use pm::{MidiMessage, DeviceInfo};

use trigger::Trigger;
use effects::effect::{Effect, MonoGroup};
use absolute_sleep::AbsoluteSleep;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use virtual_midi::{VirtualMidiOutput, MidiLightPatch};

pub struct Patch {
    name: String,
    effects: Vec<(Box<Trigger>, Box<Effect>)>,
    last_midi_events: HashMap<Trigger, MidiMessage>,
    program: u8,
    midi_light_patch: Option<MidiLightPatch>
}

impl Patch {
    pub fn new<S: Into<String>>(name: S, effects: Vec<(Box<Trigger>, Box<Effect>)>, program: u8,
                                midi_light_patch: Option<MidiLightPatch>) -> Patch {
        Patch {
            name: name.into(),
            effects: effects,
            last_midi_events: HashMap::new(),
            program: program,
            midi_light_patch: midi_light_patch
        }
    }

    pub fn init(&mut self, virtual_midi_out: &Arc<Mutex<VirtualMidiOutput>>) {
        if let Some(ref midi_light_patch) = self.midi_light_patch {
            virtual_midi_out.lock().unwrap().reconfigure(midi_light_patch);
        }
    }

    pub fn update_from(&mut self, patch: Patch, virtual_midi_out: &Arc<Mutex<VirtualMidiOutput>>) {
        let running_triggers: Vec<Box<Trigger>> = self.effects.iter()
            .filter_map(|&(ref trigger, ref eff)| if eff.is_running() { Some(trigger.clone()) } else { None }).collect();
        self.stop_running_effects();
        self.name = patch.name;
        self.effects = patch.effects;
        self.midi_light_patch = patch.midi_light_patch;
        self.init(virtual_midi_out);
        self.program = patch.program;
        let absolute_sleep = AbsoluteSleep::new();
        for &mut (ref trigger, ref mut effect) in &mut self.effects {
            if running_triggers.contains(trigger) {
                if let Some(last_midi_message) = self.last_midi_events.get(trigger) {
                    effect.start(*last_midi_message, absolute_sleep, virtual_midi_out)
                }
            }
        }
    }

    pub fn on_midi_event(&mut self, device: &DeviceInfo, midi_message: MidiMessage,
                         virtual_midi_out: &Arc<Mutex<VirtualMidiOutput>>) {
        // println!("Patch.on_midi_event {:?}  {:?}", device, midi_message);
        for &mut (ref mut _t, ref mut effect) in self.effects.as_mut_slice() {
            effect.on_midi_event(device, midi_message);
        }
        let triggered_effect_indices: Vec<usize> = (0..self.effects.len()).filter(|&i| self.effects[i].0.is_triggered(device, midi_message)).collect();
        if !triggered_effect_indices.is_empty() {
            let triggered_mono_groups: Vec<MonoGroup> = triggered_effect_indices.iter().map(|&i| self.effects[i].1.mono_group()).collect();
            for triggered_mono_group in triggered_mono_groups {
                self.stop_running_effects_in_mono_group(triggered_mono_group);
            }
            let absolute_sleep = AbsoluteSleep::new();
            for triggered_index in triggered_effect_indices {
                self.last_midi_events.insert(*self.effects[triggered_index].0.clone(), midi_message);
                self.effects[triggered_index].1.start(midi_message, absolute_sleep, virtual_midi_out);
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
