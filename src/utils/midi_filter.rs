use crate::utils::range_mapper::ValueRange;
use portmidi::MidiMessage;
use crate::utils::is_note_on;
use crate::utils::is_note_off;
use crate::utils::is_control_change;
use crate::utils::range_mapper::ALL_REAL_NOTES;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum FilterType {
    Note,
    NoteOn,
    Control(u8),
}

#[derive(Debug)]
pub struct MidiFilter {
    pub device: String,
    pub range: ValueRange,
    pub filter_type: FilterType,
}

impl MidiFilter {
    pub fn note(device: &str, note: u8) -> Self {
        MidiFilter {
            device: device.to_string(),
            range: (note, note),
            filter_type: FilterType::Note,
        }
    }

    pub fn notes<S: Into<String>>(device: S) -> Self {
        MidiFilter {
            device: device.into(),
            range: ALL_REAL_NOTES,
            filter_type: FilterType::Note,
        }
    }

    pub fn notes_on<S: Into<String>>(device: S) -> Self {
        MidiFilter {
            device: device.into(),
            range: ALL_REAL_NOTES,
            filter_type: FilterType::NoteOn,
        }
    }

    pub fn control<S: Into<String>>(device: S, control: u8) -> Self {
        MidiFilter {
            device: device.into(),
            range: (0, 127),
            filter_type: FilterType::Control(control),
        }
    }

    pub fn matches(&self, device: &str, midi_message: MidiMessage) -> bool {
        if !device.contains(&self.device) {
            return false;
        }

        let matches_type = match self.filter_type {
            FilterType::Note => is_note_on(midi_message) || is_note_off(midi_message),
            FilterType::NoteOn => is_note_on(midi_message),
            FilterType::Control(control) => is_control_change(midi_message) && midi_message.data1 == control
        };
        if !matches_type {
            return false;
        }

        self.range.0 <= midi_message.data1 && midi_message.data1 <= self.range.1
    }
}
