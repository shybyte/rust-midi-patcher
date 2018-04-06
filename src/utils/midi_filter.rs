use utils::range_mapper::ValueRange;
use pm::MidiMessage;
use utils::is_note_on;
use utils::is_note_off;
use utils::is_control_change;

pub enum FilterType {
    Note,
    #[allow(dead_code)]
    Control,
}

pub struct MidiFilter {
    pub device: String,
    pub range: ValueRange,
    pub filter_type: FilterType,
}

impl MidiFilter {
    pub fn matches(&self, device: &str, midi_message: MidiMessage) -> bool {
        if !device.contains(&self.device) {
            return false;
        }

        let matches_type = match self.filter_type {
            FilterType::Note => is_note_on(midi_message) || is_note_off(midi_message),
            FilterType::Control => is_control_change(midi_message)
        };
        if !matches_type {
            return false;
        }

        return self.range.0 >= midi_message.data1 && midi_message.data1 <= self.range.1;
    }
}