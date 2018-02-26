use pm::{MidiMessage};
use std::sync::{Arc, Mutex};
use absolute_sleep::AbsoluteSleep;

use virtual_midi::VirtualMidiOutput;

pub type DeviceName = str;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ThreadCommand {
    Stop
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum MonoGroup {
    Note,
    ControlIndex(u8),
}

pub trait Effect {
    fn on_midi_event(&mut self, _device: &DeviceName, _midi_message: MidiMessage) {}
    fn start(&mut self, midi_message: MidiMessage, absolute_sleep: AbsoluteSleep, virtual_midi_out: &Arc<Mutex<VirtualMidiOutput>>);
    fn stop(&mut self);
    fn is_running(&self) -> bool;
    fn mono_group(&self) -> MonoGroup;
}

