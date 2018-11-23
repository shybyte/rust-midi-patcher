use crate::microkorg::*;
use crate::midi_devices::*;
use crate::config::Config;
use crate::patch::Patch;
use crate::trigger::Trigger;
use crate::effects::control_forwarder::ControlForwarder;
use crate::effects::control_sequencer::ControlSequencer;
use std::time::Duration;
use crate::utils::Boxable;
use crate::midi_beat_tracker::MidiBeatTracker;
use crate::utils::midi_filter::FilterType;
use crate::utils::midi_filter::MidiFilter;
use crate::effects::control_sequencer::NoteDuration;
use crate::effects::control_to_pitch_forwarder::ControlToPitchForwarder;


static PITCH: &str = "PITCH";


pub fn diktator(_config: &Config) -> Patch {
    Patch::new("diktator",
               vec![
                   (
                       Trigger::never().boxit(),
                       ControlForwarder::new(
                           EXPRESS_PEDAL, USB_MIDI_ADAPTER, CUTOFF).boxit()
                   ),
                   (
                       Trigger::new(HAND_SONIC, 74).boxit(),
                       ControlSequencer::new(
                           USB_MIDI_ADAPTER,
                           OSC2_SEMITONE,
                           vec![126, 114, 96, 78, 126, 126, 114, 114],
                           64,
                           NoteDuration::Relative(16), // Duration::from_millis(100)?,
                       ).boxit()
                   ),

                   (
                       Trigger::new(HAND_SONIC, 74).boxit(),
                       ControlSequencer::new(
                           PITCH,
                           OSC2_SEMITONE,
                           vec![126, 114, 96, 78],
                           64,
                           NoteDuration::Relative(16), // Duration::from_millis(100)?,
                       ).boxit()
                   ),

                   (
                       Box::new(Trigger::never()),
                       Box::new(ControlToPitchForwarder::new(
                           PITCH, THROUGH_PORT))
                   ),
               ],
               43, // A64
               None)
        .beat_tracker(MidiBeatTracker::new(
            MidiFilter {
                device: USB_MIDI_ADAPTER.to_string(),
                range: (10, 127),
                filter_type: FilterType::NoteOn,
            }, Duration::from_millis(2000))
        )
}
