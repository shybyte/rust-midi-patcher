use crate::config::Config;
use crate::effects::control_forwarder::ControlForwarder;
use crate::effects::control_sequence_stepper::ControlSequenceStepper;
use crate::effects::harmony_drum::HarmonyDrum;
use crate::effects::pedal_melody::PedalMelody;
use crate::effects::button_melody::{ButtonMelody, HarmonyButtonMelody};
use crate::effects::button_melody_sustaining::ButtonMelodySustaining;
use crate::microkorg::*;
use crate::midi_devices::*;
use crate::midi_notes::*;
use crate::patch::Patch;
use std::time::Duration;
use crate::trigger::Trigger;
use crate::effects::control_to_pitch_forwarder::ControlToPitchForwarder;
use crate::effects::midi_forwarder::MidiForwarder;
use crate::utils::midi_filter::MidiFilter;
use crate::utils::midi_filter::FilterType;
use crate::utils::range_mapper::RangeToRangeMapper;
use crate::midi_devices::HAND_SONIC;
use crate::utils::add_notes;

pub fn wahrheit(_config: &Config) -> Patch {
    Patch::new("wahrheit",
               vec![
                   (
                       Box::new(Trigger::new(HAND_SONIC, 74)),
                       Box::new(ControlSequenceStepper::new(
                           USB_MIDI_ADAPTER, OSC2_SEMITONE, &[64, 95])
                           .with_reset_filter(MidiFilter::note(HAND_SONIC, 70))
                       )
                   ),
                   (
                       Box::new(Trigger::never()),
                       Box::new(ControlForwarder::new_with_value_mapper(
                           EXPRESS_PEDAL, USB_MIDI_ADAPTER, MOD,
                           RangeToRangeMapper::new((0, 255), (0, 255)),
                       ))
                   ),
                   (
                       Box::new(Trigger::new(HAND_SONIC, 74)),
                       Box::new(HarmonyDrum::new(
                           USB_MIDI_ADAPTER, THROUGH_PORT, (C2, C4), vec![12],
                           Duration::from_millis(100),
                           1.0,
                           Duration::from_secs(3600)))
                   )
               ],
               48, // A71
               None)
}


pub fn liebeslieder(_config: &Config) -> Patch {
    Patch::new("liebeslieder",
               vec![
                   (
                       Box::new(Trigger::never()),
                       Box::new(PedalMelody::new_with_treshholds(
                           EXPRESS_PEDAL, "LOOP", &[C5, D5, E5], 1, 127))
                   ),
//                   (
//                       Box::new(Trigger::never()),
//                       Box::new(ButtonMelody::new(
//                           "LOOP", C5, USB_MIDI_ADAPTER,
//                           add(vec![0, 4, -1, -3, 0, 4, 7, 5], C5),
//                           Duration::from_secs(2)))
//                   ),
                   (
                       Box::new(Trigger::never()),
                       Box::new(ButtonMelodySustaining::new(
                           "LOOP", C5, E5, USB_MIDI_ADAPTER,
                           add_notes(vec![0, 4, -1, -3, 0, 4, 7, 5], C5),
                           Duration::from_secs(5), Duration::from_millis(250)))
                   ),
               ],
               27 // 44
               , None)
}

pub fn sicherheitskopie(_config: &Config) -> Patch {
    Patch::new(
        "sicherheitskopie",
        vec![
            (
                Box::new(Trigger::never()),
                Box::new(PedalMelody::new(
                    EXPRESS_PEDAL, "LOOP", &[C5]))
            ),
            (
                Box::new(Trigger::new("LOOP", C5)),
                Box::new(ButtonMelody::new(
                    "LOOP", vec![C5], USB_MIDI_ADAPTER,
                    vec![5, 3, 1, 5, 3, 1, 8, 7, 5], CIS5 as i8,
                    Duration::from_secs(2)))
            ),
        ],
        30  // 47
        , None)
}


pub fn young(_config: &Config) -> Patch {
    Patch::new("young",
               vec![
                   (
                       Box::new(Trigger::new(HAND_SONIC, 61)),
                       Box::new(HarmonyDrum::new(USB_MIDI_ADAPTER, USB_MIDI_ADAPTER, (C2, D3), vec![0],
                                                 Duration::from_millis(100),
                                                 1.0,
                                                 Duration::from_secs(3600)))
                   ),
                   (
                       Box::new(Trigger::new(HAND_SONIC, 63)),
                       Box::new(HarmonyDrum::new(
                           USB_MIDI_ADAPTER, USB_MIDI_ADAPTER, (C2, C5), vec![7, 12, 19],
                           Duration::from_millis(100),
                           0.2,
                           Duration::from_secs(3600)))
                   ),
                   (
                       Box::new(Trigger::never()),
                       Box::new(ControlForwarder::new_with_value_mapper(
                           EXPRESS_PEDAL, USB_MIDI_ADAPTER, CUTOFF,
                           RangeToRangeMapper::new((0, 255), (10, 255)),
                       ))
                   ),
                   (
                       Box::new(Trigger::never()),
                       Box::new(ControlToPitchForwarder::new(
                           EXPRESS_PEDAL, THROUGH_PORT))
                   ),
               ],
               28, // a45
               None)
}

pub fn enddzeit(_config: &Config) -> Patch {
    Patch::new("endzeit",
               vec![
//                   (
//                       Box::new(Trigger::new(HAND_SONIC, 60)),
//                       Box::new(HarmonyDrum::new(USB_MIDI_ADAPTER, USB_MIDI_ADAPTER, (C2, C6), vec![-12],
//                                                 Duration::from_millis(100),
//                                                 1.0,
//                                                 Duration::from_millis(0), Duration::from_secs(3600)))
//                   ),
(
    Box::new(Trigger::new(HAND_SONIC, 64)),
    Box::new(HarmonyDrum::new(
        USB_MIDI_ADAPTER, USB_MIDI_ADAPTER, (C2, C6), vec![7, 12, 19],
        Duration::from_millis(100),
        0.2,
        Duration::from_secs(3600)))
),
(
    Box::new(Trigger::never()),
    Box::new(ControlForwarder::new_with_value_mapper(
        EXPRESS_PEDAL, USB_MIDI_ADAPTER, CUTOFF,
        RangeToRangeMapper::new((0, 255), (10, 255)),
    ))
),
//(
//    Box::new(Trigger::never()),
//    Box::new(ControlToPitchForwarder::new(
//        EXPRESS_PEDAL, THROUGH_PORT))
//),
               ],
               28, // a45
               None)
}



pub fn system(_config: &Config) -> Patch {
    Patch::new("system",
               vec![
                   (
                       Box::new(Trigger::never()),
                       Box::new(MidiForwarder::new(
                           MidiFilter {
                               device: HAND_SONIC.to_string(),
                               range: (10, 127),
                               filter_type: FilterType::Note,
                           }, THROUGH_PORT)
                       )
                   ),
                   (
                       Box::new(Trigger::never()),
                       Box::new(HarmonyButtonMelody {
                           harmony_input_filter: MidiFilter {
                               device: HAND_SONIC.to_string(),
                               range: (10, 127),
                               filter_type: FilterType::Note,
                           },
                           stop_signal_filter: Some(MidiFilter::note("PEDAL_NOTE", 1)),
                           button_melodies: vec![
                               ButtonMelody::new(
                                   HAND_SONIC, vec![1], THROUGH_PORT,
                                   vec![12, 19], C4 as i8,
                                   Duration::from_secs(2))
                                   .with_reset_filter(MidiFilter::note(HAND_SONIC, A4)),
                               ButtonMelody::new(
                                   HAND_SONIC, vec![5], THROUGH_PORT,
                                   vec![19], C4 as i8,
                                   Duration::from_secs(2))
                           ],
                           active: true,
                       }
                       )
                   ),
//                   (
//                       Box::new(Trigger::new(HAND_SONIC, 1)),
//                       Box::new(ControlSequenceStepper::new(
//                           USB_MIDI_ADAPTER, CONTROL2, &[1, 2, 3, 4, 10,20,30]))
//                   ),
//                   (
//                       Box::new(Trigger::never()),
//                       Box::new(HarmonyButtonMelody {
//                           harmony_input_filter: MidiFilter {
//                               device: HAND_SONIC.to_string(),
//                               range: (10, 127),
//                               filter_type: FilterType::Note,
//                           },
//                           button_melodies: vec![
//                               ButtonMelody::new(
//                                   HAND_SONIC, 1, THROUGH_PORT,
//                                   vec![19], C4 as i8,
//                                   Duration::from_secs(2),
//                               ),
//                               ButtonMelody::new(
//                                   HAND_SONIC, 2, THROUGH_PORT,
//                                   vec![-12], C4 as i8,
//                                   Duration::from_secs(2),
//                               )
//                           ],
//                       }
//                       )
//                   )
               ],
               51, // A74
               None)
}
