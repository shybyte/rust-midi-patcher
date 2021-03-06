use portmidi::{MidiMessage};
use std::sync::{Arc, Mutex};
use crate::absolute_sleep::AbsoluteSleep;

use crate::virtual_midi::VirtualMidiOutput;
use std::time::Duration;

pub type DeviceName = str;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ThreadCommand {
    Stop
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum MonoGroup {
    None,
    Note,
    ControlIndex(u8),
}

pub trait Effect {
    fn on_midi_event(&mut self, _device: &DeviceName, _midi_message: MidiMessage, _virtual_midi_out: &Arc<Mutex<VirtualMidiOutput>>) {}
    fn start(&mut self, _midi_message: MidiMessage, _absolute_sleep: AbsoluteSleep, _virtual_midi_out: &Arc<Mutex<VirtualMidiOutput>>) {}
    fn stop(&mut self, _virtual_midi_out: &Arc<Mutex<VirtualMidiOutput>>) {}
    fn is_running(&self) -> bool {false}
    fn set_beat_duration(&mut self, _duration: Duration) {}
    fn mono_group(&self) -> MonoGroup {MonoGroup::None}
}

