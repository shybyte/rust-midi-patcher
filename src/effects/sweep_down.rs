use crate::pm::{MidiMessage};
use std::sync::{Arc, Mutex, mpsc};
use std::sync::mpsc::{Sender};
use std::time::Duration;
use std::thread;
use crate::absolute_sleep::AbsoluteSleep;
use crate::utils::{control_change};
use crate::effects::effect::{Effect, MonoGroup, ThreadCommand};
use crate::virtual_midi::VirtualMidiOutput;


pub struct SweepDown {
    output_device: String,
    min_value: u8,
    control_index: u8,
    mono_group: MonoGroup,
    sender: Option<Sender<ThreadCommand>>,
}

impl SweepDown {
    pub fn new(output_device: &str, min_value: u8, control_index: u8) -> SweepDown {
        SweepDown {
            output_device: output_device.to_string(),
            min_value,
            control_index,
            mono_group: MonoGroup::ControlIndex(control_index),
            sender: None,
        }
    }
}

impl Effect for SweepDown {
    fn start(&mut self, midi_message: MidiMessage, absolute_sleep: AbsoluteSleep,
             virtual_midi_out: &Arc<Mutex<VirtualMidiOutput>>) {
        if self.sender.is_some() {
            self.stop(virtual_midi_out);
        }
        let (tx, rx) = mpsc::channel();
        self.sender = Some(tx);
        let velocity = f32::from(midi_message.data2);
        let mut control_value: f32 = velocity;
        let control_index = self.control_index;
        let min_value = f32::from(self.min_value);
        let mut absolute_sleep = absolute_sleep;
        let out_device = self.output_device.clone();
        let virtual_midi_out = Arc::clone(virtual_midi_out);

        thread::spawn(move || {
            println!("start sweep down = {:?}", midi_message);

            while control_value >= min_value {
                control_change(&out_device, &virtual_midi_out, control_index, control_value as u8);
                println!("sweep = {:?}", control_value);
                control_value = control_value - 1.0 - (velocity / 50.0);
                absolute_sleep.sleep(Duration::from_millis(20));
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
        self.mono_group
    }
}


