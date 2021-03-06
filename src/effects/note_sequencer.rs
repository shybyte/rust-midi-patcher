use crate::pm::{MidiMessage};
use std::sync::{Arc, Mutex, mpsc};
use std::sync::mpsc::Sender;
use std::time::Duration;
use std::thread;
use crate::absolute_sleep::AbsoluteSleep;
use crate::effects::effect::{Effect, MonoGroup, ThreadCommand};
use crate::virtual_midi::VirtualMidiOutput;
use crate::utils::play_note_on;
use crate::utils::play_note_off;


pub struct NoteSequencer {
    output_device: String,
    notes: Arc<Vec<u8>>,
    velocity: u8,
    time_per_note: Duration,
    sender: Option<Sender<ThreadCommand>>,
}

impl NoteSequencer {
    pub fn new(output_device: &str, notes: Vec<u8>, time_per_note: Duration, velocity: u8) -> NoteSequencer {
        NoteSequencer {
            output_device: output_device.to_string(),
            notes: Arc::new(notes),
            velocity,
            time_per_note,
            sender: None,
        }
    }
}

impl Effect for NoteSequencer {
    fn start(&mut self, midi_message: MidiMessage, absolute_sleep: AbsoluteSleep, virtual_midi_out: &Arc<Mutex<VirtualMidiOutput>>) {
        if self.sender.is_some() {
            self.stop(virtual_midi_out);
        }
        let (tx, rx) = mpsc::channel();
        self.sender = Some(tx);
        let notes = Arc::clone(&self.notes);
        let velocity = self.velocity;
        let time_per_note = self.time_per_note;
        let mut absolute_sleep = absolute_sleep;
        let output_name = self.output_device.clone();
        let virtual_midi_out = Arc::clone(virtual_midi_out);

        thread::spawn(move || {
            //       let start_time = SystemTime::now();
            println!("start sequence = {:?}", midi_message);
            //       println!("start time = {:?}", start_time);

            for &note in notes.iter() {
                //                println!("play note = {:?}", note);
                //                let elapsed = start_time.elapsed().unwrap();
                //                let millis = elapsed.as_secs() * 1_000 + (elapsed.subsec_nanos() / 1_000_000) as u64;
                //                let divergence = millis % 220;
                //                if (divergence < 110 && divergence > 5) || (divergence > 110 && divergence < 215) {
                //                    println!("elapsed = {:?} {:?}", divergence, millis);
                //                }

                play_note_on(&output_name, &virtual_midi_out, note, velocity);

                absolute_sleep.sleep(time_per_note / 4);

                play_note_off(&output_name, &virtual_midi_out, note);

                let r = rx.try_recv();
                if let Ok(ThreadCommand::Stop) = r {
                    println!("got stop command = {:?}", midi_message.data1);
                    break;
                }

                absolute_sleep.sleep(time_per_note * 3 / 4);

                let r = rx.try_recv();
                if let Ok(ThreadCommand::Stop) = r {
                    println!("got stop command = {:?}", midi_message.data1);
                    break;
                }
            }
        });
    }

    fn stop(&mut self, _virtual_midi_out: &Arc<Mutex<VirtualMidiOutput>>) {
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

