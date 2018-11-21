use crate::pm::{MidiMessage};
use std::sync::{Arc, Mutex, mpsc};
use std::sync::mpsc::{Sender};
use std::time::Duration;
use std::thread;
use crate::absolute_sleep::AbsoluteSleep;
use crate::utils::{control_change};
use crate::effects::effect::{Effect, MonoGroup, ThreadCommand};
use crate::virtual_midi::VirtualMidiOutput;



pub struct ControlSequencer {
    output_device: String,
    control_index: u8,
    values: Arc<Vec<u8>>,
    stop_value: u8,
    mono_group: MonoGroup,
    time_per_note: Duration,
    sender: Option<Sender<ThreadCommand>>,
}

impl ControlSequencer {
    pub fn new(output_device: &str,control_index: u8, values: Vec<u8>, stop_value: u8, time_per_note: Duration) -> ControlSequencer {
        ControlSequencer {
            output_device: output_device.to_string(),
            control_index,
            values: Arc::new(values),
            stop_value,
            time_per_note,
            sender: None,
            mono_group: MonoGroup::ControlIndex(control_index)
        }
    }
}

impl Effect for ControlSequencer {
    fn start(&mut self, midi_message: MidiMessage, absolute_sleep: AbsoluteSleep, virtual_midi_out: &Arc<Mutex<VirtualMidiOutput>>) {
        if self.sender.is_some() {
            self.stop(virtual_midi_out);
        }
        let (tx, rx) = mpsc::channel();
        self.sender = Some(tx);
        let values = Arc::clone(&self.values);
        let control_index = self.control_index;
        let time_per_note = self.time_per_note;
        let stop_value = self.stop_value;
        let mut absolute_sleep = absolute_sleep;

        let out_device = self.output_device.clone();
        let virtual_midi_out = Arc::clone(virtual_midi_out);

        thread::spawn(move || {
            println!("start sequence = {:?}", midi_message);
            for &value in values.iter() {
                control_change(&out_device, &virtual_midi_out, control_index, value);
                absolute_sleep.sleep(time_per_note);

                let r = rx.try_recv();
                if let Ok(ThreadCommand::Stop) = r {
                    println!("got stop command = {:?}", midi_message.data1);
                    break;
                }
            }
            control_change(&out_device, &virtual_midi_out, control_index, stop_value);
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
