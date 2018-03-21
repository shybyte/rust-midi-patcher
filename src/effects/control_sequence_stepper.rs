use absolute_sleep::AbsoluteSleep;
use effects::effect::{Effect, MonoGroup};
use pm::MidiMessage;
use std::sync::{Arc, Mutex};
use utils::control_change;
use virtual_midi::VirtualMidiOutput;


pub struct ControlSequenceStepper {
    output_device: String,
    control_index: u8,
    values: Vec<u8>,
    value_index: usize,
    mono_group: MonoGroup,
}

impl ControlSequenceStepper {
    pub fn new(output_device: &str,control_index: u8, values: &[u8]) -> Self {
        ControlSequenceStepper {
            output_device: output_device.to_string(),
            control_index: control_index,
            values: values.to_vec(),
            value_index: 0,
            mono_group: MonoGroup::ControlIndex(control_index)
        }
    }
}

impl Effect for ControlSequenceStepper {
    fn start(&mut self, _midi_message: MidiMessage, _absolute_sleep: AbsoluteSleep, virtual_midi_out: &Arc<Mutex<VirtualMidiOutput>>) {
        let value = self.values[self.value_index];
        control_change(&self.output_device, &virtual_midi_out, self.control_index, value);
        self.value_index = (self.value_index + 1) % self.values.len();
    }

    fn mono_group(&self) -> MonoGroup {
        self.mono_group
    }
}