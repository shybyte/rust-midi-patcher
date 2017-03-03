extern crate portmidi as pm;


use std::thread;
use pm::{MidiMessage, DeviceInfo, OutputPort, PortMidi};
use std::sync::{Arc, Mutex, mpsc};
use std::sync::mpsc::{Sender};
use std::time::Duration;

use trigger::Trigger;
use effect::{Effect, NoteSequencer};

const BUF_LEN: usize = 1024;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum ThreadCommand {
    Stop
}


pub struct Patch {
    effects: Vec<Box<Effect>>
}

impl Patch {
    pub fn new(context: &PortMidi) -> Patch {
        let eff = NoteSequencer::new(context, vec![45, 45, 47, 47]);
        Patch {
            effects: vec![Box::new(eff)]
        }
    }

    pub fn on_midi_event(&mut self, device: &DeviceInfo, midi_message: MidiMessage) {
        //        println!("Before {:?}  {:?}", device, midi_message);
        if device.name().contains("VMPK") {
            if midi_message.status == 144 {
                //let first = self.effects.iter_mut().find(|x| true);
                if let Some(eff) = self.effects.first_mut() {
                    eff.start(midi_message);
                }
            } else {
                //                let mut output_port = self.output_port.lock().unwrap();
                //                output_port.write_message(midi_message).unwrap();
            }
        }
    }
}

fn send_midi(output_port_mutex2: &mut Arc<Mutex<OutputPort>>, m: MidiMessage) {
    let mut output_port = output_port_mutex2.lock().unwrap();
    output_port.write_message(m).unwrap();
}