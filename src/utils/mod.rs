use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::io;
use std::iter;
use std::ops::Add;
use std::sync::{Arc, Mutex};
use crate::pm::{MidiMessage};
use crate::virtual_midi::VirtualMidiOutput;


pub mod range_mapper;
pub mod midi_filter;

pub fn repeated<T: Clone>(pattern: &[T], times: usize) -> Vec<T> {
    concat(iter::repeat(pattern.to_vec()).take(times).collect())
}

pub fn add<T>(mut xs: Vec<T>, y: T) -> Vec<T>
    where T: Copy + Add<T, Output=T> {
    for x in &mut xs {
        *x = *x + y;
    }
    xs
}

pub fn concat<T: Clone>(input: Vec<Vec<T>>) -> Vec<T> {
    input.into_iter().flat_map(|x| x).collect()
}

pub fn send_midi(output_name: &str, midi_output: &Arc<Mutex<VirtualMidiOutput>>, m: MidiMessage) {
    let mut output_port = midi_output.lock().unwrap();
    output_port.play(output_name, m);
}

pub fn control_change(output_name: &str, midi_output: &Arc<Mutex<VirtualMidiOutput>>, control_index: u8, value: u8) {
    let note_on = MidiMessage {
        status: 0xB0,
        data1: control_index,
        data2: value,
    };

    send_midi(output_name, midi_output, note_on);
}

pub fn pitch_wheel(output_name: &str, midi_output: &Arc<Mutex<VirtualMidiOutput>>, data1: u8, data2: u8) {
    let message = MidiMessage {
        status: 224,
        data1,
        data2,
    };

    send_midi(output_name, midi_output, message);
}

pub fn play_note_on(output_name: &str, midi_output: &Arc<Mutex<VirtualMidiOutput>>, note: u8, velocity: u8) {
    let note_on = MidiMessage {
        status: 0x90,
        data1: note,
        data2: velocity,
    };

    midi_output.lock().unwrap().play_and_visualize(output_name, note_on);
}

pub fn play_note_off(output_name: &str, midi_output: &Arc<Mutex<VirtualMidiOutput>>, note: u8) {
    let note_off = MidiMessage {
        status: 0x80,
        data1: note,
        data2: 0x40,
    };

    midi_output.lock().unwrap().play_and_visualize(output_name, note_off);
}

pub fn read_file<P: AsRef<Path>>(file_name: P) -> Result<String, io::Error> {
    let mut file = File::open(file_name).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

pub fn is_note_on(midi_message: MidiMessage) -> bool {
    (midi_message.status == 0x90 || (midi_message.status == 0x99 && midi_message.data2 > 0))
}

pub fn is_note_off(midi_message: MidiMessage) -> bool {
    (midi_message.status == 0x80)
}

pub fn is_control_change(midi_message: MidiMessage) -> bool {
    (midi_message.status == 0xB0)
}

pub fn add_notes(xs: Vec<i16>, y: u8) -> Vec<u8> {
    xs.into_iter().map(|x| (x + i16::from(y)) as u8).collect()
}

pub trait Boxable {
    fn boxit(self) -> Box<Self>;
}

impl <T> Boxable for T {
    fn boxit(self) -> Box<Self> {
        Box::new(self)
    }
}
