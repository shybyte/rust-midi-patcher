use pm::{MidiMessage, OutputPort};
use std::sync::{Arc, Mutex};
use absolute_sleep::AbsoluteSleep;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ThreadCommand {
    Stop
}

pub trait Effect {
    fn start(&mut self, output_ports: &[Arc<Mutex<OutputPort>>], midi_message: MidiMessage, absolute_sleep: AbsoluteSleep);
    fn stop(&mut self);
    fn is_running(&self) -> bool;
}

