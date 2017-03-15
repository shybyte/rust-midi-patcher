use pm::{MidiMessage, OutputPort};
use std::sync::{Arc, Mutex, mpsc};
use std::sync::mpsc::{Sender};
use std::time::Duration;
use std::thread;
use absolute_sleep::AbsoluteSleep;
use utils::send_midi;
use effects::effect::{Effect, MonoGroup, ThreadCommand};

pub struct SweepDown {
    output_device: String,
    min_value: u8,
    control_index: u8,
    mono_group: MonoGroup,
    sender: Option<Sender<ThreadCommand>>,
    output_port: Option<Arc<Mutex<OutputPort>>>,
}

impl SweepDown {
    pub fn new(output_device: &str, min_value: u8, control_index: u8) -> SweepDown {
        SweepDown {
            output_device: output_device.to_string(),
            min_value: min_value,
            control_index: control_index,
            mono_group: MonoGroup::ControlIndex(control_index),
            sender: None,
            output_port: None
        }
    }
}

impl Effect for SweepDown {
    fn start(&mut self, output_ports: &[Arc<Mutex<OutputPort>>], midi_message: MidiMessage, absolute_sleep: AbsoluteSleep) {
        if self.sender.is_some() {
            self.stop();
        }
        let mut output_port_mutex: Arc<Mutex<OutputPort>> = output_ports.iter()
            .find(|p| p.lock().unwrap().device().name().contains(&self.output_device)).unwrap().clone();
        self.output_port = Some(output_port_mutex.clone());
        let (tx, rx) = mpsc::channel();
        self.sender = Some(tx);
        let velocity: f32 = midi_message.data2 as f32;
        let mut control_value: f32 = velocity;
        let control_index = self.control_index;
        let min_value: f32 = self.min_value as f32;
        let mut absolute_sleep = absolute_sleep;
        thread::spawn(move || {
            println!("start sequence = {:?}", midi_message);

            while control_value >= min_value {
                control_change(&mut output_port_mutex, control_index, control_value as u8);
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


fn control_change(output_port_mutex: &mut Arc<Mutex<OutputPort>>, control_index: u8, value: u8) {
    let note_on = MidiMessage {
        status: 0xB0,
        data1: control_index,
        data2: value,
    };

    send_midi(output_port_mutex, note_on);
}

