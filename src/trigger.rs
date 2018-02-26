use effects::effect::DeviceName;
use pm::{MidiMessage};
use utils::is_note_on;


#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct Trigger {
    device: String,
    note: u8
}


impl Trigger {
    pub fn new(device: &str, note: u8) -> Trigger {
        Trigger { device: device.to_string(), note: note }
    }

    pub fn is_triggered(&self, device: &DeviceName, midi_message: MidiMessage) -> bool {
        device.contains(&self.device) && is_note_on(midi_message) && midi_message.data1 == self.note
    }
}
