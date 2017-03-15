use pm::{MidiMessage, OutputPort};
use std::sync::{Arc, Mutex, mpsc};
use std::sync::mpsc::{Sender};
use std::time::Duration;
use std::thread;
use std::collections::HashSet;
use absolute_sleep::AbsoluteSleep;
use utils::send_midi;
use effects::effect::{Effect, MonoGroup, ThreadCommand};


pub struct NoteSequencer {
    output_device: String,
    notes: Arc<Vec<u8>>,
    velocity: u8,
    time_per_note: Duration,
    sender: Option<Sender<ThreadCommand>>,
    output_port: Option<Arc<Mutex<OutputPort>>>,
    playing_notes: Arc<Mutex<HashSet<u8>>>,
}

impl NoteSequencer {
    pub fn new(output_device: &str, notes: Vec<u8>, time_per_note: Duration, velocity: u8) -> NoteSequencer {
        NoteSequencer {
            output_device: output_device.to_string(),
            notes: Arc::new(notes),
            velocity: velocity,
            time_per_note: time_per_note,
            sender: None,
            output_port: None,
            playing_notes: Arc::new(Mutex::new(HashSet::new()))
        }
    }
}

impl Effect for NoteSequencer {
    fn start(&mut self, output_ports: &[Arc<Mutex<OutputPort>>], midi_message: MidiMessage, absolute_sleep: AbsoluteSleep) {
        if self.sender.is_some() {
            self.stop();
        }
        let mut output_port_mutex: Arc<Mutex<OutputPort>> = output_ports.iter()
            .find(|p| p.lock().unwrap().device().name().contains(&self.output_device)).unwrap().clone();
        self.output_port = Some(output_port_mutex.clone());
        let (tx, rx) = mpsc::channel();
        self.sender = Some(tx);
        let notes = self.notes.clone();
        let playing_notes = self.playing_notes.clone();
        let velocity = self.velocity;
        let time_per_note = self.time_per_note;
        let mut absolute_sleep = absolute_sleep;
        thread::spawn(move || {
            println!("start sequence = {:?}", midi_message);

            for &note in notes.iter() {
                //                println!("play note = {:?}", note);

                playing_notes.lock().unwrap().insert(note);
                play_note_on(&mut output_port_mutex, note, velocity);


                absolute_sleep.sleep(time_per_note / 2);

                play_note_off(&mut output_port_mutex, note);
                playing_notes.lock().unwrap().remove(&note);

                let r = rx.try_recv();
                if let Ok(ThreadCommand::Stop) = r {
                    println!("got stop command = {:?}", midi_message.data1);
                    break;
                }

                absolute_sleep.sleep(time_per_note / 2);

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

    fn mono_group(&self) -> MonoGroup {
        MonoGroup::Note
    }
}

impl Drop for NoteSequencer {
    fn drop(&mut self) {
        if let Some(ref mut output_port) = self.output_port {
            for &note in self.playing_notes.lock().unwrap().iter() {
                play_note_off(output_port, note);
            }
        }
    }
}


fn play_note_on(output_port_mutex: &mut Arc<Mutex<OutputPort>>, note: u8, velocity: u8) {
    let note_on = MidiMessage {
        status: 0x90,
        data1: note,
        data2: velocity,
    };

    send_midi(output_port_mutex, note_on);
}

fn play_note_off(output_port_mutex: &mut Arc<Mutex<OutputPort>>, note: u8) {
    let note_off = MidiMessage {
        status: 0x80,
        data1: note,
        data2: 0x40,
    };

    send_midi(output_port_mutex, note_off);
}
