use microkorg::*;
use midi_devices::*;
use config::Config;
use patch::Patch;
use trigger::Trigger;
use effects::control_forwarder::ControlForwarder;
use effects::control_sequencer::ControlSequencer;
use std::time::Duration;
use utils::Boxable;


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
