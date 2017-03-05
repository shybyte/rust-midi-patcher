use pm::{MidiMessage, OutputPort};

use std::sync::{Arc, Mutex, mpsc};
use std::sync::mpsc::{Sender};
use std::time::Duration;

use std::thread;


#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum ThreadCommand {
    Stop
}


pub trait Effect {
    fn start(&mut self, midi_message: MidiMessage);
    fn stop(&mut self);
    fn is_running(&self) -> bool;
}

pub struct NoteSequencer {
    output_port: Arc<Mutex<OutputPort>>,
    notes: Arc<Vec<u8>>,
    velocity: u8,
    time_per_note: Duration,
    sender: Option<Sender<ThreadCommand>>
}

impl NoteSequencer {
    pub fn new(output_port: Arc<Mutex<OutputPort>>, notes: Vec<u8>, time_per_note: Duration, velocity: u8) -> NoteSequencer {
        println!("notes = {:?}", notes);
        NoteSequencer {
            output_port: output_port,
            notes: Arc::new(notes),
            velocity: velocity,
            time_per_note: time_per_note,
            sender: None,
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
        let notes = self.notes.clone();
        let velocity = self.velocity;
        let time_per_note = self.time_per_note;
        thread::spawn(move || {
            println!("start sequence = {:?}", midi_message);

            for &note in notes.iter() {
//                println!("play note = {:?}", note);

                let note_on = MidiMessage {
                    status: 0x90,
                    data1: note,
                    data2: velocity,
                };

                send_midi(&mut output_port_mutex, note_on);

                thread::sleep(time_per_note / 2);


                let note_off = MidiMessage {
                    status: 0x80,
                    data1: note,
                    data2: 0x40,
                };

                send_midi(&mut output_port_mutex, note_off);

                let r = rx.try_recv();
                if let Ok(ThreadCommand::Stop) = r {
                    println!("got stop command = {:?}", midi_message.data1);
                    break;
                }

                thread::sleep(time_per_note / 2);

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

    fn is_running(&self) -> bool {
        self.sender.is_some()
    }
}

fn send_midi(output_port_mutex2: &mut Arc<Mutex<OutputPort>>, m: MidiMessage) {
    let mut output_port = output_port_mutex2.lock().unwrap();
    output_port.write_message(m).unwrap();
}