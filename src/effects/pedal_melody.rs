use effects::effect::Effect;
use effects::effect::DeviceName;
use pm::MidiMessage;
use std::sync::{Arc, Mutex};
use virtual_midi::VirtualMidiOutput;
use utils::play_note_on;
use utils::play_note_off;

const UPPER_THRESHOLD: u8 = 124;
const LOWER_THRESHOLD: u8 = 4;

type ControlValue = u8;
type Note = u8;

pub struct PedalMelody {
    input_device: String,
    output_device: String,
    note_mappings: Vec<(ControlValue, Note)>,
    prev_note_value: u8,
}

impl PedalMelody {
    pub fn new(input_device: &str, output_device: &str, notes: &[Note]) -> PedalMelody {
        PedalMelody {
            input_device: input_device.to_string(),
            output_device: output_device.to_string(),
            note_mappings: create_notes_mapping(notes),
            prev_note_value: 0,
        }
    }
    pub fn new_with_treshholds(input_device: &str, output_device: &str, notes: &[Note], lower_threshold: u8, upper_threshold: u8) -> PedalMelody {
        PedalMelody {
            input_device: input_device.to_string(),
            output_device: output_device.to_string(),
            note_mappings: create_notes_mapping_internal(notes, lower_threshold, upper_threshold),
            prev_note_value: 0,
        }
    }
}

impl Effect for PedalMelody {
    fn on_midi_event(&mut self, device: &DeviceName,
                     midi_message: MidiMessage,
                     virtual_midi_out: &Arc<Mutex<VirtualMidiOutput>>) {
        if device.contains(&self.input_device) {
            // eprintln!("midi_message = {:?}", midi_message);
            let control_value = midi_message.data2;
            let &(_, note) = self.note_mappings.iter().find(|&&(threshold, _note)| control_value >= threshold).unwrap();
            if note != self.prev_note_value {
                eprintln!("Play {} {}", note, self.output_device);
                if { note > 0 } {
                    play_note_on(&self.output_device, &virtual_midi_out, note, 127);
                }
                if { self.prev_note_value > 0 } {
                    play_note_off(&self.output_device, &virtual_midi_out, self.prev_note_value);
                }
            }
            self.prev_note_value = note;
        }
    }
}


fn create_notes_mapping(notes: &[Note]) -> Vec<(ControlValue, Note)> {
    create_notes_mapping_internal(notes, LOWER_THRESHOLD, UPPER_THRESHOLD)
}

fn create_notes_mapping_internal(notes: &[Note], lower_threshold: u8, upper_threshold: u8) -> Vec<(ControlValue, Note)> {
    if notes.is_empty() {
        panic!("Missing notes");
    }
    let mut result = vec![(UPPER_THRESHOLD, notes[0])];

    match notes.len() {
        1 | 2 => { result.push((0, *notes.get(1).unwrap_or(&0))); }
        _ => {
            let &(middle_notes, tail) = &notes[1..].split_at(notes.len() - 2);
            let middle_note_range_size = (upper_threshold - lower_threshold) / (middle_notes.len()) as u8;
            for (i, &note) in middle_notes.iter().enumerate() {
                result.push((upper_threshold - (i as u8 + 1) * middle_note_range_size, note));
            }
            result.push((0, tail[0]));
        }
    }

    result
}


//#[test]
//#[should_panic]
//fn test_create_notes_mapping_empty_notes() {
//    create_notes_mapping(&vec![]);
//}

#[test]
fn test_create_notes_mapping_1() {
    assert_eq!(create_notes_mapping(&vec![1]), vec![
        (UPPER_THRESHOLD, 1),
        (0, 0)
    ]);
}

#[test]
fn test_create_notes_mapping_2() {
    assert_eq!(create_notes_mapping(&vec![1, 2]), vec![
        (UPPER_THRESHOLD, 1),
        (0, 2),
    ]);
}

#[test]
fn test_create_notes_mapping_3() {
    assert_eq!(create_notes_mapping(&vec![1, 2, 3]), vec![
        (UPPER_THRESHOLD, 1),
        (LOWER_THRESHOLD, 2),
        (0, 3),
    ]);
}

#[test]
fn test_create_notes_mapping_4() {
    assert_eq!(create_notes_mapping(&vec![1, 2, 3, 4]), vec![
        (UPPER_THRESHOLD, 1),
        (64, 2),
        (LOWER_THRESHOLD, 3),
        (0, 4),
    ]);
}

#[test]
fn test_create_notes_mapping_5() {
    assert_eq!(create_notes_mapping(&vec![1, 2, 3, 4, 5]), vec![
        (UPPER_THRESHOLD, 1),
        (84, 2),
        (44, 3),
        (LOWER_THRESHOLD, 4),
        (0, 5),
    ]);
}



