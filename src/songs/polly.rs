use std::time::Duration;
use patch::Patch;
use trigger::Trigger;
use effects::sweep_down::{SweepDown};
use effects::control_sequencer::{ControlSequencer};
use midi_devices::*;
use utils::{repeated};
use microkorg::*;


pub fn create_polly() -> Patch {
    let arp_seq = repeated(&[78, 96, 114, 126], 2);
    let arp = ControlSequencer::new(DEFAULT_OUT_DEVICE, OSC2_SEMITONE, arp_seq, 64, Duration::from_millis(30));
    let sweep_down = SweepDown::new(DEFAULT_OUT_DEVICE, 30, CUTOFF);

    Patch::new(vec![
        (Box::new(Trigger::new(SAMPLE_PAD, 41)), Box::new(sweep_down)),
        (Box::new(Trigger::new(SAMPLE_PAD, 40)), Box::new(arp))
    ], 10)
}
