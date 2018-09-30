use utils::range_mapper::ValueRange;
use pm::MidiMessage;
use utils::is_note_on;
use utils::is_note_off;
use utils::is_control_change;
use utils::range_mapper::ALL_REAL_NOTES;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum FilterType {
    Note,
    NoteOn,
    #[allow(dead_code)]
    Control,
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

    pub fn matches(&self, device: &str, midi_message: MidiMessage) -> bool {
        if !device.contains(&self.device) {
            return false;
        }

        let matches_type = match self.filter_type {
            FilterType::Note => is_note_on(midi_message) || is_note_off(midi_message),
            FilterType::NoteOn => is_note_on(midi_message),
            FilterType::Control => is_control_change(midi_message)
        };
        if !matches_type {
            return false;
        }

        return self.range.0 <= midi_message.data1 && midi_message.data1 <= self.range.1;
    }
}
