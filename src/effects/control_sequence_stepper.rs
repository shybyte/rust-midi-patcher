use crate::absolute_sleep::AbsoluteSleep;
use crate::effects::effect::{Effect, MonoGroup};
use crate::pm::MidiMessage;
use std::sync::{Arc, Mutex};
use crate::utils::control_change;
use crate::virtual_midi::VirtualMidiOutput;
use crate::utils::midi_filter::MidiFilter;
use crate::effects::effect::DeviceName;


pub struct ControlSequenceStepper {
    output_device: String,
    control_index: u8,
    values: Vec<u8>,
    value_index: usize,
    mono_group: MonoGroup,
    reset_filter: Option<MidiFilter>,
}

impl ControlSequenceStepper {
    pub fn new(output_device: &str,control_index: u8, values: &[u8]) -> Self {
        ControlSequenceStepper {
            output_device: output_device.to_string(),
            control_index: control_index,
            values: values.to_vec(),
            value_index: 0,
            mono_group: MonoGroup::ControlIndex(control_index),
            reset_filter: None
        }
    }

    pub fn with_reset_filter(mut self, filter: MidiFilter) -> Self {
        self.reset_filter = Some(filter);
        self
    }


}

impl Effect for ControlSequenceStepper {
    fn on_midi_event(&mut self, device: &DeviceName, midi_message: MidiMessage, _virtual_midi_out: &Arc<Mutex<VirtualMidiOutput>>) {
        if let Some(ref stop_signal_filter) = self.reset_filter {
            if stop_signal_filter.matches(device, midi_message) {
                self.value_index = 0;
            }
        }
    }


    fn start(&mut self, _midi_message: MidiMessage, _absolute_sleep: AbsoluteSleep, virtual_midi_out: &Arc<Mutex<VirtualMidiOutput>>) {
        let value = self.values[self.value_index];
        control_change(&self.output_device, &virtual_midi_out, self.control_index, value);
        self.value_index = (self.value_index + 1) % self.values.len();
    }

    fn mono_group(&self) -> MonoGroup {
        self.mono_group
    }
}
