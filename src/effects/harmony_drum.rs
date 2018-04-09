use absolute_sleep::AbsoluteSleep;
use effects::effect::{Effect, MonoGroup};
use effects::effect::DeviceName;
use midi_notes::*;
use pm::{MidiMessage};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use utils::is_note_on;
use virtual_midi::VirtualMidiOutput;
use utils::play_note_on;
use utils::play_note_off;


pub struct HarmonyDrum {
    note_range: (u8, u8),
    notes: Vec<i8>,
    notes_index: usize,
    harmony_input_device: String,
    output_device: String,
    current_note: u8,
    note_duration: Duration,
    debounce_duration: Duration,
    reset_duration: Duration,
    last_timestamp: Instant,
}

impl HarmonyDrum {
    pub fn new(harmony_input_device: &str, output_device: &str,
               note_range: (u8, u8), notes: Vec<i8>,
               note_duration: Duration,
               debounce_duration: Duration,
               reset_duration: Duration) -> HarmonyDrum {
        HarmonyDrum {
            note_range,
            notes_index: 0,
            notes,
            harmony_input_device: harmony_input_device.to_string(),
            output_device: output_device.to_string(),
            current_note: C3,
            note_duration,
            debounce_duration,
            reset_duration,
            last_timestamp: Instant::now(),
        }
    }
}

impl Effect for HarmonyDrum {
    fn on_midi_event(&mut self, device: &DeviceName, midi_message: MidiMessage, _virtual_midi_out: &Arc<Mutex<VirtualMidiOutput>>) {
        if device.contains(&self.harmony_input_device) && is_note_on(midi_message)
            && self.note_range.0 <= midi_message.data1 && midi_message.data1 <= self.note_range.1 {
            self.current_note = midi_message.data1;
            println!("===> got harmony input midi_message = {:?}", midi_message);
        }
    }

    fn start(&mut self, midi_message: MidiMessage, absolute_sleep: AbsoluteSleep,
             virtual_midi_out: &Arc<Mutex<VirtualMidiOutput>>) {
        let timestamp = Instant::now();
        let duration_since_last_event = timestamp - self.last_timestamp;
        self.last_timestamp = timestamp;

        if { duration_since_last_event < self.debounce_duration} {
            return;
        } else if { duration_since_last_event > self.reset_duration} {
            self.notes_index = 0;
        }

        eprintln!("Note Index {:?}", self.notes_index);

        self.last_timestamp = Instant::now();
        let mut absolute_sleep = absolute_sleep;
        let played_note = (self.current_note as i8 + self.notes[self.notes_index]) as u8;
        self.notes_index = (self.notes_index + 1) % self.notes.len();
        play_note_on(&self.output_device, virtual_midi_out, played_note, midi_message.data2);
        let virtual_midi_out_clone = Arc::clone(virtual_midi_out);
        let out_device = self.output_device.clone();
        let note_duration = self.note_duration.clone();
        thread::spawn(move || {
            println!("play harmony drum {:?} {:?}", midi_message, played_note);
            absolute_sleep.sleep(note_duration);
            play_note_off(&out_device, &virtual_midi_out_clone, played_note);
        });
    }

    fn stop(&mut self, _virtual_midi_out: &Arc<Mutex<VirtualMidiOutput>>) {}

    fn is_running(&self) -> bool {
        false
    }

    fn mono_group(&self) -> MonoGroup {
        MonoGroup::Note
    }
}


