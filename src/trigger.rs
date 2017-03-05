use pm::{MidiMessage, DeviceInfo};


#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Trigger {
    device: String,
    note: u8
}


impl Trigger {
    pub fn new(device: &str, note: u8) -> Trigger {
        Trigger { device: device.to_string(), note: note }
    }

    pub fn is_triggered(&self, device: &DeviceInfo, midi_message: MidiMessage) -> bool {
        device.name().contains(&self.device) &&
            midi_message.status == 0x90 &&
            midi_message.data1 == self.note
    }
}
