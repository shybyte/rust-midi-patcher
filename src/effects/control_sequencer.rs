use pm::{MidiMessage, OutputPort};
use std::sync::{Arc, Mutex, mpsc};
use std::sync::mpsc::{Sender};
use std::time::Duration;
use std::thread;
use absolute_sleep::AbsoluteSleep;
use utils::{control_change};
use effects::effect::{Effect, MonoGroup, ThreadCommand};
use chan;
use view::main_view::ToViewEvents;


pub struct ControlSequencer {
    output_device: String,
    control_index: u8,
    values: Arc<Vec<u8>>,
    stop_value: u8,
    mono_group: MonoGroup,
    time_per_note: Duration,
    sender: Option<Sender<ThreadCommand>>,
    output_port: Option<Arc<Mutex<OutputPort>>>,
}

impl ControlSequencer {
    pub fn new(output_device: &str,control_index: u8, values: Vec<u8>, stop_value: u8, time_per_note: Duration) -> ControlSequencer {
        ControlSequencer {
            output_device: output_device.to_string(),
            control_index: control_index,
            values: Arc::new(values),
            stop_value: stop_value,
            time_per_note: time_per_note,
            sender: None,
            output_port: None,
            mono_group: MonoGroup::ControlIndex(control_index)
        }
    }
}

impl Effect for ControlSequencer {
    fn start(&mut self, output_ports: &[Arc<Mutex<OutputPort>>], midi_message: MidiMessage, absolute_sleep: AbsoluteSleep, _to_view_tx: &chan::Sender<ToViewEvents>) {
        if self.sender.is_some() {
            self.stop();
        }
        let mut output_port_mutex: Arc<Mutex<OutputPort>> = output_ports.iter()
            .find(|p| p.lock().unwrap().device().name().contains(&self.output_device)).unwrap().clone();
        self.output_port = Some(output_port_mutex.clone());
        let (tx, rx) = mpsc::channel();
        self.sender = Some(tx);
        let values = self.values.clone();
        let control_index = self.control_index;
        let time_per_note = self.time_per_note;
        let stop_value = self.stop_value;
        let mut absolute_sleep = absolute_sleep;
        thread::spawn(move || {
            println!("start sequence = {:?}", midi_message);
            for &value in values.iter() {
                control_change(&mut output_port_mutex, control_index, value);
                absolute_sleep.sleep(time_per_note);

                let r = rx.try_recv();
                if let Ok(ThreadCommand::Stop) = r {
                    println!("got stop command = {:?}", midi_message.data1);
                    break;
                }
            }
            control_change(&mut output_port_mutex, control_index, stop_value);
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
        self.mono_group
    }
}

impl Drop for ControlSequencer {
    fn drop(&mut self) {
        if let Some(ref mut output_port) = self.output_port {
            control_change(output_port, self.control_index, self.stop_value);
        }
    }
}


