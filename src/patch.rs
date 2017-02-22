extern crate portmidi as pm;

use std::thread;
use pm::{MidiMessage, DeviceInfo, OutputPort, PortMidi};
use std::sync::{Arc, Mutex, MutexGuard, mpsc};
use std::sync::mpsc::{Sender, Receiver};
use std::time::Duration;

const BUF_LEN: usize = 1024;

enum ThreadCommand {
    STOP
}

pub struct Patch {
    notes: Vec<u8>,
    pub output_port: Arc<Mutex<OutputPort>>,
    tx: Sender<ThreadCommand>,
    rx: Receiver<ThreadCommand>
}

impl Patch {
    pub fn new(context: &PortMidi) -> Patch {
        let first_out_device = context.devices().unwrap().into_iter()
            .find(|dev| dev.is_output())
            .unwrap();
        let output_port = context.output_port(first_out_device, BUF_LEN).unwrap();
        let (tx, rx) = mpsc::channel();
        Patch {
            notes: Vec::new(),
            output_port: Arc::new(Mutex::new(output_port)),
            tx: tx,
            rx: rx
        }
    }

    pub fn on_midi_event(&mut self, device: &DeviceInfo, midi_message: MidiMessage) {
        println!("Before {:?}  {:?}", device, midi_message);
        if device.name().contains("VMPK") {
            if midi_message.status == 144 && midi_message.data1 == 45 {
                let mut output_port_mutex: Arc<Mutex<OutputPort>> = self.output_port.clone();
                thread::spawn(move || {
                    println!("Hello from a thread!");
                    println!("midi_message = {:?}", midi_message);

                    for i in 0..4 {
                        println!("loop midi_message = {:?}", i);

                        let note_on = MidiMessage {
                            status: 0x90,
                            data1: 45,
                            data2: 100,
                        };

                        send_midi(&mut output_port_mutex, midi_message);

                        thread::sleep(Duration::from_millis(1 as u64 * 400));

                        let note_on = MidiMessage {
                            status: 0x80,
                            data1: 45,
                            data2: 100,
                        };

                        send_midi(&mut output_port_mutex, midi_message);

                        thread::sleep(Duration::from_millis(1 as u64 * 400));
                    }
                });
            } else {
                let mut output_port = self.output_port.lock().unwrap();
                output_port.write_message(midi_message).unwrap();
            }
        }
    }
}

fn send_midi(output_port_mutex2: &mut Arc<Mutex<OutputPort>>, m: MidiMessage) {
    let mut output_port = output_port_mutex2.lock().unwrap();
    output_port.write_message(m).unwrap();
}