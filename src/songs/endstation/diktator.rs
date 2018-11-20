use crate::microkorg::*;
use crate::midi_devices::*;
use crate::config::Config;
use crate::patch::Patch;
use crate::trigger::Trigger;
use crate::effects::control_forwarder::ControlForwarder;
use crate::effects::control_sequencer::ControlSequencer;
use std::time::Duration;
use crate::utils::Boxable;


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
                           vec![126, 114, 96, 78],
                           64,
                           Duration::from_millis(100),
                       ).boxit()
                   ),
               ],
               43, // A64
               None)
}
