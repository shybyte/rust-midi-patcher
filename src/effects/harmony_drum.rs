use pm::{MidiMessage, OutputPort, DeviceInfo};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread;
use absolute_sleep::AbsoluteSleep;
use utils::send_midi;
use effects::effect::{Effect, MonoGroup};
use chan;
use midi_notes::*;
use view::main_view::ToViewEvents;

pub struct HarmonyDrum {
    harmony_input_device: String,
    output_device: String,
    output_port: Option<Arc<Mutex<OutputPort>>>,
}

impl HarmonyDrum {
    pub fn new(harmony_input_device: &str, output_device: &str) -> HarmonyDrum {
        HarmonyDrum {
            harmony_input_device: harmony_input_device.to_string(),
            output_device: output_device.to_string(),
            output_port: None
        }
    }
}

impl Effect for HarmonyDrum {
    fn on_midi_event(&mut self, device: &DeviceInfo, midi_message: MidiMessage) {
        if device.name().contains(&self.harmony_input_device) {
            println!("===> got harmony input midi_message = {:?}", midi_message);
        }
    }

    fn start(&mut self, output_ports: &[Arc<Mutex<OutputPort>>], midi_message: MidiMessage, absolute_sleep: AbsoluteSleep, _to_view_tx: &chan::Sender<ToViewEvents>) {
        let mut output_port_mutex: Arc<Mutex<OutputPort>> = output_ports.iter()
            .find(|p| p.lock().unwrap().device().name().contains(&self.output_device)).unwrap().clone();
        self.output_port = Some(output_port_mutex.clone());
        let mut absolute_sleep = absolute_sleep;
        let played_note = C4;
        play_note_on(&mut output_port_mutex, played_note, 100);
        thread::spawn(move || {
            println!("play harmony drum {:?} {:?}", midi_message, played_note);
            absolute_sleep.sleep(Duration::from_millis(50));
            play_note_off(&mut output_port_mutex, played_note);
        });
    }

    fn stop(&mut self) {
    }

    fn is_running(&self) -> bool {
        false
    }

    fn mono_group(&self) -> MonoGroup {
        MonoGroup::Note
    }
}

impl Drop for HarmonyDrum {
    fn drop(&mut self) {
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
