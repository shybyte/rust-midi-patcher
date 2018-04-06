#![allow(dead_code)]


use std::collections::vec_deque::VecDeque;
use midi_message::MidiMessage;


struct MidiBeatTracker {
    midi_events: VecDeque<u64>,
    beat_duration: f64,
    beat_duration_range: f64,
}

impl MidiBeatTracker {
    fn new(default_beat_duration: f64, beat_duration_range: f64) -> Self {
        MidiBeatTracker {
            midi_events: VecDeque::new(),
            beat_duration: default_beat_duration,
            beat_duration_range,
        }
    }

    pub fn on_midi_event(&mut self, time_stamp: u64, midi_event: MidiMessage) {
        if let MidiMessage::NoteOn(_, _, _) = midi_event {
            self.midi_events.push_back(time_stamp);
        }
    }

    pub fn beat_duration(&self) -> f64 {
        self.beat_duration
    }
}


#[cfg(test)]
mod tests {
    use midi_beat_tracker::MidiBeatTracker;
    use midi_message::MidiMessage;

    #[test]
    fn test() {
        let mut tracker = MidiBeatTracker::new(1.0, 0.5);
        tracker.on_midi_event(0, MidiMessage::NoteOn(0, 45, 56));
        assert_eq!(tracker.beat_duration(), 1.0);
    }
}
