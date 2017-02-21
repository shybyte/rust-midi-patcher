extern crate portmidi as pm;

use pm::{MidiMessage, DeviceInfo, OutputPort, PortMidi};

const BUF_LEN: usize = 1024;

pub struct Patch {
    notes: Vec<u8>,
    pub output_port: OutputPort
}

impl Patch {
    pub fn new(context: &PortMidi) -> Patch {
        let first_out_device = context.devices().unwrap().into_iter()
            .find(|dev| dev.is_output())
            .unwrap();
        let output_port = context.output_port(first_out_device, BUF_LEN).unwrap();
        Patch { notes: Vec::new(), output_port: output_port }
    }

    pub fn on_midi_event(&mut self, device: &DeviceInfo, midi_message: MidiMessage) {
        println!("Before {:?}  {:?}", device, midi_message);
        if device.name().contains("VMPK") {
            self.output_port.write_message(midi_message).unwrap();
        }
    }
}