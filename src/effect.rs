use pm::{MidiMessage, DeviceInfo, OutputPort, PortMidi};

use std::sync::{Arc, Mutex, mpsc};
use std::sync::mpsc::{Sender};
use std::time::Duration;

use std::thread;

const BUF_LEN: usize = 1024;


#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum ThreadCommand {
    Stop
}


pub trait Effect {
    fn start(&mut self, midi_message: MidiMessage);
    fn stop(&mut self);
}

pub struct NoteSequencer {
    output_port: Arc<Mutex<OutputPort>>,
    notes: Vec<u8>,
    sender: Option<Sender<ThreadCommand>>
}

impl NoteSequencer {
    pub fn new(context: &PortMidi, notes: Vec<u8>) -> NoteSequencer {
        let first_out_device = context.devices().unwrap().into_iter()
            .find(|dev| dev.is_output())
            .unwrap();
        let output_port = context.output_port(first_out_device, BUF_LEN).unwrap();
        NoteSequencer {
            output_port: Arc::new(Mutex::new(output_port)),
            notes: notes,
            sender: None
        }
    }
}

impl Effect for NoteSequencer {
    fn start(&mut self, midi_message: MidiMessage) {
        if self.sender.is_some() {
            self.stop();
        }
        let mut output_port_mutex: Arc<Mutex<OutputPort>> = self.output_port.clone();
        let (tx, rx) = mpsc::channel();
        self.sender = Some(tx);
        thread::spawn(move || {
            println!("midi_message = {:?}", midi_message);

            for i in 0..8 {
                println!("loop midi_message = {:?}", i);

                let note_on = MidiMessage {
                    status: 0x90,
                    data1: midi_message.data1,
                    data2: 100,
                };

                send_midi(&mut output_port_mutex, note_on);

                thread::sleep(Duration::from_millis(1 as u64 * 100));

                let note_off = MidiMessage {
                    status: 0x80,
                    data1: midi_message.data1,
                    data2: 100,
                };

                send_midi(&mut output_port_mutex, note_off);

                let r = rx.try_recv();
                if let Ok(ThreadCommand::Stop) = r {
                    println!("got stop command = {:?}", midi_message.data1);
                    break;
                }

                thread::sleep(Duration::from_millis(1 as u64 * 100));

                let r = rx.try_recv();
                if let Ok(ThreadCommand::Stop) = r {
                    println!("got stop command = {:?}", midi_message.data1);
                    break;
                }
            }
        });
    }

    fn stop(&mut self) {
        if let Some(ref tx) = self.sender {
            tx.send(ThreadCommand::Stop).ok();
        }
        self.sender = None;
    }
}

fn send_midi(output_port_mutex2: &mut Arc<Mutex<OutputPort>>, m: MidiMessage) {
    let mut output_port = output_port_mutex2.lock().unwrap();
    output_port.write_message(m).unwrap();
}