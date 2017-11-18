use pm::{MidiMessage, DeviceInfo};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread;
use absolute_sleep::AbsoluteSleep;
use effects::effect::{Effect, MonoGroup};
use midi_notes::*;
use utils::is_note_on;
use virtual_midi::VirtualMidiOutput;


pub struct HarmonyDrum {
    note_range: (u8, u8),
    notes: Vec<u8>,
    notes_index: usize,
    harmony_input_device: String,
    output_device: String,
    current_note: u8,
}

impl HarmonyDrum {
    pub fn new(harmony_input_device: &str, output_device: &str, note_range: (u8, u8), notes: Vec<u8>) -> HarmonyDrum {
        HarmonyDrum {
            note_range,
            notes_index: 0,
            notes,
            harmony_input_device: harmony_input_device.to_string(),
            output_device: output_device.to_string(),
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

    fn start(&mut self, midi_message: MidiMessage, absolute_sleep: AbsoluteSleep,
             virtual_midi_out: &Arc<Mutex<VirtualMidiOutput>>) {
        let mut absolute_sleep = absolute_sleep;
        let played_note = self.current_note + self.notes[self.notes_index];
        self.notes_index = (self.notes_index + 1) % self.notes.len();
        play_note_on(&self.output_device, &virtual_midi_out, played_note, midi_message.data2);
        let virtual_midi_out_clone = virtual_midi_out.clone();
        let out_device = self.output_device.clone();
        thread::spawn(move || {
            println!("play harmony drum {:?} {:?}", midi_message, played_note);
            absolute_sleep.sleep(Duration::from_millis(100));
            play_note_off(&out_device, &virtual_midi_out_clone, played_note);
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


fn play_note_on(output_name: &str, midi_output: &Arc<Mutex<VirtualMidiOutput>>, note: u8, velocity: u8) {
    let note_on = MidiMessage {
        status: 0x90,
        data1: note,
        data2: velocity,
    };

    midi_output.lock().unwrap().play_and_visualize(output_name, note_on);
}

fn play_note_off(output_name: &str, midi_output: &Arc<Mutex<VirtualMidiOutput>>, note: u8) {
    let note_off = MidiMessage {
        status: 0x80,
        data1: note,
        data2: 0x40,
    };

    midi_output.lock().unwrap().play_and_visualize(output_name, note_off);
}


