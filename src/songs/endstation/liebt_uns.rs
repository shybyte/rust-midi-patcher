use crate::config::Config;
use crate::effects::button_melody::{ButtonMelody, HarmonyButtonMelody};
use crate::effects::button_melody_sustaining::ButtonMelodySustaining;
use crate::effects::midi_forwarder::MidiForwarder;
use crate::effects::pedal_melody::PedalMelody;
use crate::midi_devices::HAND_SONIC;
use crate::midi_devices::*;
use crate::midi_notes::*;
use crate::patch::Patch;
use std::time::Duration;
use crate::utils::add;
use crate::utils::Boxable;
use crate::utils::midi_filter::MidiFilter;
use crate::effects::control_forwarder::ControlForwarder;
use crate::midi_devices::EXPRESS_PEDAL;
use crate::midi_devices::USB_MIDI_ADAPTER;
use crate::microkorg::CUTOFF;
use crate::microkorg::VOLUME;

static PEDAL_NOTE: &str = "PEDAL_NOTE";
static BASE_NOTE: &str = "BASE_NOTE";
static LARS_OUT: &str = "LARS_OUT";

pub fn liebt_uns(_config: &Config) -> Patch {
    Patch::new_simple(
        "liebt-uns",
        vec![
            // Pedal -> PEDAL_NOTE
            PedalMelody::new(EXPRESS_PEDAL, PEDAL_NOTE, &[C5, 0, 1]).boxit(),

            // PEDAL_NOTE -> BASE_NOTE -> THROUGH_PORT
//            ButtonMelodySustaining::new(
//                PEDAL_NOTE,
//                C5,
//                1,
//                THROUGH_PORT,
//                //                           add(vec![0, 7, 3, 8], A3),
//                add(vec![12, 11, 7, 9, 12, 11, 0, 5, 12, 11, 7, 0], C5),
//                Duration::from_secs(2),
//                Duration::from_millis(250),
//            ).boxit(),
//            ButtonMelodySustaining::new(
//                PEDAL_NOTE,
//                C5,
//                1,
//                BASE_NOTE,
//                //                           add(vec![0, 7, 3, 8], A3),
//                add(vec![0, 7, 3, 8], A3),
//                Duration::from_secs(2),
//                Duration::from_millis(250),
//            ).boxit(),
//            MidiForwarder::new(MidiFilter::notes(BASE_NOTE), THROUGH_PORT).boxit(),
//            MidiForwarder::new(MidiFilter::notes(BASE_NOTE), K_BOARD).boxit(),
//            MidiForwarder::new(MidiFilter::notes(LARS_OUT), K_BOARD).boxit(),
            MidiForwarder::new(MidiFilter::notes(LARS_OUT), USB_MIDI_ADAPTER).boxit(),
            ControlForwarder::new(EXPRESS_PEDAL, USB_MIDI_ADAPTER, CUTOFF).boxit(),


            // (BASE_NOTE, PEDAL_NOTE)-> HarmonyButtonMelody -> USB_MIDI_ADAPTER
            HarmonyButtonMelody {
                harmony_input_filter: MidiFilter::notes_on(USB_MIDI_ADAPTER),
                stop_signal_filter: Some(MidiFilter::note(PEDAL_NOTE, 1)),
                button_melodies: vec![
                    ButtonMelody::new(
                        HAND_SONIC,
                        vec![74],
                        LARS_OUT,
                        vec![0],
                        A4 as i8,
                        Duration::from_secs(2),
                    ),
                    ButtonMelody::new(
                        HAND_SONIC,
                        vec![60],
                        LARS_OUT,
                        vec![12],
                        A4 as i8,
                        Duration::from_secs(2),
                    ),
                    ButtonMelody::new(
                        HAND_SONIC,
                        vec![64],
                        LARS_OUT,
                        vec![7],
                        A4 as i8,
                        Duration::from_secs(2),
                    ),
                ],
                active: false,
            }.boxit()
        ],
        113, // b72
    )
}

pub fn liebt_uns2(_config2: &Config) -> Patch {
    Patch::new_simple(
        "liebt-uns",
        vec![
            // Pedal -> PEDAL_NOTE
            PedalMelody::new(EXPRESS_PEDAL, PEDAL_NOTE, &[C5, 0, 1]).boxit(),

            // PEDAL_NOTE -> BASE_NOTE -> THROUGH_PORT
            ButtonMelodySustaining::new(
                PEDAL_NOTE,
                C5,
                1,
                THROUGH_PORT,
                //                           add(vec![0, 7, 3, 8], A3),
                add(vec![12, 11, 7, 9, 12, 11, 0, 5, 12, 11, 7, 0], C5),
                Duration::from_secs(2),
                Duration::from_millis(250),
            ).boxit(),
            ButtonMelodySustaining::new(
                PEDAL_NOTE,
                C5,
                1,
                BASE_NOTE,
                //                           add(vec![0, 7, 3, 8], A3),
                add(vec![0, 7, 3, 8], A3),
                Duration::from_secs(2),
                Duration::from_millis(250),
            ).boxit(),
//            MidiForwarder::new(MidiFilter::notes(BASE_NOTE), THROUGH_PORT).boxit(),
            MidiForwarder::new(MidiFilter::notes(BASE_NOTE), K_BOARD).boxit(),
//            MidiForwarder::new(MidiFilter::notes(LARS_OUT), K_BOARD).boxit(),
            MidiForwarder::new(MidiFilter::notes(LARS_OUT), USB_MIDI_ADAPTER).boxit(),

            // (BASE_NOTE, PEDAL_NOTE)-> HarmonyButtonMelody -> USB_MIDI_ADAPTER
            HarmonyButtonMelody {
                harmony_input_filter: MidiFilter::notes_on(BASE_NOTE),
                stop_signal_filter: Some(MidiFilter::note(PEDAL_NOTE, 1)),
                button_melodies: vec![
                    ButtonMelody::new(
                        HAND_SONIC,
                        vec![74],
                        LARS_OUT,
                        vec![0],
                        A4 as i8,
                        Duration::from_secs(2),
                    ),
                    ButtonMelody::new(
                        HAND_SONIC,
                        vec![60],
                        LARS_OUT,
                        vec![12],
                        A4 as i8,
                        Duration::from_secs(2),
                    ),
                    ButtonMelody::new(
                        HAND_SONIC,
                        vec![64],
                        LARS_OUT,
                        vec![7],
                        A4 as i8,
                        Duration::from_secs(2),
                    ),
                ],
                active: false,
            }.boxit()
        ],
        113, // b72
    )
}
