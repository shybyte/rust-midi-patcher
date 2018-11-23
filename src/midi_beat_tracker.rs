#![allow(dead_code)]

use portmidi::MidiMessage;
use crate::utils::midi_filter::MidiFilter;
use std::time::Instant;
use std::time::Duration;

pub struct MidiBeatTracker {
    filter: MidiFilter,
    last_time_stamp: Option<Instant>,
    beat_duration: Duration,
    default_beat_duration: Duration,
}

impl MidiBeatTracker {
    pub fn new(filter: MidiFilter, default_beat_duration: Duration) -> Self {
        MidiBeatTracker {
            filter,
            last_time_stamp: None,
            default_beat_duration,
            beat_duration: default_beat_duration,
        }
    }

    pub fn on_midi_event(&mut self, device: &str, midi_message: MidiMessage) {
        if !self.filter.matches(device, midi_message) {
            return;
        }

        let now = Instant::now();

        if let Some(last_time_stamp) = self.last_time_stamp {
            let last_beat_duration = now - last_time_stamp;
            eprintln!("min = {:?}", self.default_beat_duration / 2);
            eprintln!("last_beat_duration = {:?}", last_beat_duration);
            if (self.default_beat_duration * 2 / 3) < last_beat_duration &&
                last_beat_duration < (3 / 2 * self.default_beat_duration) {
                eprintln!(" ===> beat_duration = {:?}", last_beat_duration);
                self.beat_duration = last_beat_duration;
            }
        }

        self.last_time_stamp = Some(now);
    }

    pub fn beat_duration(&self) -> Duration {
        self.beat_duration
    }
}
