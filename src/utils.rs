use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::io;
use std::iter;
use std::ops::Add;
use std::sync::{Arc, Mutex};
use pm::{MidiMessage, OutputPort};

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

pub fn send_midi(output_port_mutex: &mut Arc<Mutex<OutputPort>>, m: MidiMessage) {
    let mut output_port = output_port_mutex.lock().unwrap();
    output_port.write_message(m).unwrap();
}

pub fn control_change(output_port_mutex: &mut Arc<Mutex<OutputPort>>, control_index: u8, value: u8) {
    let note_on = MidiMessage {
        status: 0xB0,
        data1: control_index,
        data2: value,
    };

    send_midi(output_port_mutex, note_on);
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