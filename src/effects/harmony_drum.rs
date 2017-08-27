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
use utils::is_note_on;


pub struct HarmonyDrum {
    note_range: (u8, u8),
    notes: Vec<u8>,
    notes_index: usize,
    harmony_input_device: String,
    output_device: String,
    current_note: u8,
    output_port: Option<Arc<Mutex<OutputPort>>>,
}

impl HarmonyDrum {
    pub fn new(harmony_input_device: &str, output_device: &str, note_range: (u8, u8), notes: Vec<u8>) -> HarmonyDrum {
        HarmonyDrum {
            note_range,
            notes_index: 0,
            notes,
            harmony_input_device: harmony_input_device.to_string(),
            output_device: output_device.to_string(),
            output_port: None,
            current_note: C3
        }
    }
}

impl Effect for HarmonyDrum {
    fn on_midi_event(&mut self, device: &DeviceInfo, midi_message: MidiMessage) {
        if device.name().contains(&self.harmony_input_device) && is_note_on(midi_message)
            && self.note_range.0 <= midi_message.data1 && midi_message.data1 <= self.note_range.1 {
            self.current_note = midi_message.data1;
            println!("===> got harmony input midi_message = {:?}", midi_message);
        }
    }

    fn start(&mut self, output_ports: &[Arc<Mutex<OutputPort>>], midi_message: MidiMessage, absolute_sleep: AbsoluteSleep, _to_view_tx: &chan::Sender<ToViewEvents>) {
        let mut output_port_mutex: Arc<Mutex<OutputPort>> = output_ports.iter()
            .find(|p| p.lock().unwrap().device().name().contains(&self.output_device)).unwrap().clone();
        self.output_port = Some(output_port_mutex.clone());
        let mut absolute_sleep = absolute_sleep;
        let played_note = self.current_note + self.notes[self.notes_index];
        self.notes_index = (self.notes_index + 1) % self.notes.len();
        play_note_on(&mut output_port_mutex, played_note, midi_message.data2);
        thread::spawn(move || {
            println!("play harmony drum {:?} {:?}", midi_message, played_note);
            absolute_sleep.sleep(Duration::from_millis(100));
            play_note_off(&mut output_port_mutex, played_note);
        });
    }

    fn stop(&mut self) {}

    fn is_running(&self) -> bool {
        false
    }

    fn mono_group(&self) -> MonoGroup {
        MonoGroup::Note
    }
}

impl Drop for HarmonyDrum {
    fn drop(&mut self) {}
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
