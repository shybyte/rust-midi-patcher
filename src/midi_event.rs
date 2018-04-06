#![allow(dead_code)]

use midi_message::MidiMessage;

type DeviceId = u8;

pub struct MidiEvent {
    pub device: DeviceId,
    pub message: MidiMessage
}