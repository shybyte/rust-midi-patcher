use crate::effects::effect::DeviceName;
use crate::pm::{MidiMessage};
use crate::utils::is_note_on;


#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct Trigger {
    device: String,
    note: u8
}


impl Trigger {
    pub fn new(device: &str, note: u8) -> Trigger {
        Trigger { device: device.to_string(), note }
    }
    pub fn never() -> Trigger {
        Trigger { device: "NEVER!".to_string(), note: 0 }
    }

    pub fn is_triggered(&self, device: &DeviceName, midi_message: MidiMessage) -> bool {
        device.contains(&self.device) && is_note_on(midi_message) && midi_message.data1 == self.note
    }
}
