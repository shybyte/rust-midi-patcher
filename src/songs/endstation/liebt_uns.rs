use config::Config;
use effects::button_melody::{ButtonMelody, HarmonyButtonMelody};
use effects::button_melody_sustaining::ButtonMelodySustaining;
use effects::midi_forwarder::MidiForwarder;
use effects::pedal_melody::PedalMelody;
use midi_devices::*;
use midi_devices::HAND_SONIC;
use midi_notes::*;
use patch::Patch;
use std::time::Duration;
use trigger::Trigger;
use utils::add;
use utils::midi_filter::FilterType;
use utils::midi_filter::MidiFilter;

static PEDAL_NOTE: &str = "PEDAL_NOTE";
static BASE_NOTE: &str = "BASE_NOTE";

pub fn liebt_uns(_config: &Config) -> Patch {
    Patch::new("liebt uns",
               vec![
                   (
                       Box::new(Trigger::never()),
                       Box::new(PedalMelody::new(
                           EXPRESS_PEDAL, PEDAL_NOTE, &[C5, 0, 1]))
                   ),
                   (
                       Box::new(Trigger::never()),
                       Box::new(ButtonMelodySustaining::new(
                           PEDAL_NOTE, C5, 1, BASE_NOTE,
//                           add(vec![0, 7, 3, 8], A3),
                           add(vec![12, 11, 7, 9, 12, 11, 0, 5, 12, 11, 7], C5),
                           Duration::from_secs(4), Duration::from_millis(250)))
                   ),
//                   (
//                       Box::new(Trigger::never()),
//                       Box::new(ButtonMelodySustaining::new(
//                           "LOOP", C5, E5, USB_MIDI_ADAPTER,
//                           add(vec![0, 4, -1, -3, 0, 4, 7, 5], C5),
//                           Duration::from_secs(5), Duration::from_millis(250)))
//                   )
//                   (
//                       Box::new(Trigger::never()),
//                       Box::new(MidiForwarder::new(
//                           MidiFilter {
//                               device: HAND_SONIC.to_string(),
//                               range: (10, 127),
//                               filter_type: FilterType::Note,
//                           }, THROUGH_PORT)
//                       )
//                   ),
                   (
                       Box::new(Trigger::never()),
                       Box::new(MidiForwarder::new(MidiFilter::notes(BASE_NOTE), THROUGH_PORT))
                   ),
                   (
                       Box::new(Trigger::never()),
                       Box::new(HarmonyButtonMelody {
                           harmony_input_filter: MidiFilter::notes_on(BASE_NOTE),
                           stop_signal_filter: Some(MidiFilter::note(PEDAL_NOTE, 1)),
                           button_melodies: vec![
                               ButtonMelody::new(
                                   HAND_SONIC, vec![74], USB_MIDI_ADAPTER,
                                   vec![0], A4 as i8,
                                   Duration::from_secs(2)),
                           ],
                           active: false,
                       }
                       )
                   ),
                   (
                       Box::new(Trigger::never()),
                       Box::new(HarmonyButtonMelody {
                           harmony_input_filter: MidiFilter::notes_on(BASE_NOTE),
                           stop_signal_filter: Some(MidiFilter::note(PEDAL_NOTE, 1)),
                           button_melodies: vec![
                               ButtonMelody::new(
                                   HAND_SONIC, vec![60], USB_MIDI_ADAPTER,
                                   vec![12], A4 as i8,
                                   Duration::from_secs(2)),
                               ButtonMelody::new(
                                   HAND_SONIC, vec![64], USB_MIDI_ADAPTER,
                                   vec![7], A4 as i8,
                                   Duration::from_secs(2)),
                           ],
                           active: false,
                       }
                       )
                   ),
//                   (
//                       Box::new(Trigger::never()),
//                       Box::new(HarmonyButtonMelody {
//                           harmony_input_filter: MidiFilter {
//                               device: HAND_SONIC.to_string(),
//                               range: (10, 127),
//                               filter_type: FilterType::Note,
//                           },
//                           stop_signal_filter: Some(MidiFilter::note("PEDAL_NOTE", 1)),
//                           button_melodies: vec![
//                               ButtonMelody::new(
//                                   HAND_SONIC, vec![1], THROUGH_PORT,
//                                   vec![12, 19], C4 as i8,
//                                   Duration::from_secs(2))
//                           ],
//                           active: true,
//                       }
//                       )
//                   ),
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
               18, // 33
               None)
}

