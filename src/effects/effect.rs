use pm::{MidiMessage, OutputPort};
use std::sync::{Arc, Mutex};
use absolute_sleep::AbsoluteSleep;
use chan::{Sender};
use view::main_view::ToViewEvents;


#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ThreadCommand {
    Stop
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum MonoGroup {
    Note,
    ControlIndex(u8)
}

pub trait Effect {
    fn start(&mut self, output_ports: &[Arc<Mutex<OutputPort>>], midi_message: MidiMessage, absolute_sleep: AbsoluteSleep, to_view_tx: &Sender<ToViewEvents>);
    fn stop(&mut self);
    fn is_running(&self) -> bool;
    fn mono_group(&self) -> MonoGroup;
}

