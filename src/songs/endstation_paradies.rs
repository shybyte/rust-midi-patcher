use config::Config;
use effects::control_forwarder::ControlForwarder;
use effects::control_sequence_stepper::ControlSequenceStepper;
use effects::harmony_drum::HarmonyDrum;
use effects::pedal_melody::PedalMelody;
use effects::button_melody::{ButtonMelody, HarmonyButtonMelody};
use effects::button_melody_sustaining::ButtonMelodySustaining;
use microkorg::*;
use midi_devices::*;
use midi_notes::*;
use patch::Patch;
use std::time::Duration;
use trigger::Trigger;
use effects::control_to_pitch_forwarder::ControlToPitchForwarder;
use effects::midi_forwarder::MidiForwarder;
use utils::midi_filter::MidiFilter;
use utils::midi_filter::FilterType;
use utils::range_mapper::RangeToRangeMapper;

pub fn wahrheit(_config: &Config) -> Patch {
    Patch::new("wahrheit",
               vec![
                   (
                       Box::new(Trigger::new(SAMPLE_PAD, 38)),
                       Box::new(ControlSequenceStepper::new(
                           USB_MIDI_ADAPTER, OSC2_SEMITONE, &[64, 95]))
                   ),
                   (
                       Box::new(Trigger::never()),
                       Box::new(ControlForwarder::new_with_value_mapper(
                           EXPRESS_PEDAL, USB_MIDI_ADAPTER, CUTOFF,
                           RangeToRangeMapper::new((0, 255), (0, 255)),
                       ))
                   ),
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
                           add(vec![0, 4, -1, -3, 0, 4, 7, 5], C5),
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
                    "LOOP", C5, USB_MIDI_ADAPTER,
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
                       Box::new(Trigger::new(SAMPLE_PAD, 45)),
                       Box::new(HarmonyDrum::new(USB_MIDI_ADAPTER, USB_MIDI_ADAPTER, (C2, D3), vec![0],
                                                 Duration::from_millis(100), Duration::from_millis(0), Duration::from_secs(3600)))
                   ),
                   (
                       Box::new(Trigger::new(SAMPLE_PAD, 51)),
                       Box::new(HarmonyDrum::new(
                           USB_MIDI_ADAPTER, USB_MIDI_ADAPTER, (C2, C5), vec![7, 12, 19], Duration::from_millis(100), Duration::from_millis(0), Duration::from_secs(3600)))
                   ),
                   (
                       Box::new(Trigger::never()),
                       Box::new(ControlForwarder::new(
                           EXPRESS_PEDAL, USB_MIDI_ADAPTER, CUTOFF))
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


pub fn diktator(_config: &Config) -> Patch {
    Patch::new("diktator",
               vec![
                   (
                       Box::new(Trigger::never()),
                       Box::new(ControlForwarder::new(
                           EXPRESS_PEDAL, USB_MIDI_ADAPTER, CUTOFF))
                   ),
               ],
               43, // A64
               None)
}


pub fn liebt_uns(_config: &Config) -> Patch {
    Patch::new("liebt uns",
               vec![
                   (
                       Box::new(Trigger::never()),
                       Box::new(PedalMelody::new(
                           EXPRESS_PEDAL, "LOOP", &[C5]))
                   ),
                   (
                       Box::new(Trigger::new("LOOP", C5)),
                       Box::new(ButtonMelody::new(
                           "LOOP", C5, USB_MIDI_ADAPTER,
                           vec![4, 7, 4, 0, 4, 7, 16, 12], C4 as i8,
                           Duration::from_secs(2)))
                   ),
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
                           button_melody: ButtonMelody::new(
                               HAND_SONIC, C5, USB_MIDI_ADAPTER,
                               vec![0], 0,
                               Duration::from_secs(2)),
                       }
                       )
                   ),
               ],
               18, // 33
               None)
}


pub fn add(xs: Vec<i16>, y: u8) -> Vec<u8> {
    xs.iter().map(|x| (x + (y as i16)) as u8).collect()
}
