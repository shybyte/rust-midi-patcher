use config::Config;
use effects::button_melody::{ButtonMelody, HarmonyButtonMelody};
use effects::button_melody_sustaining::ButtonMelodySustaining;
use effects::midi_forwarder::MidiForwarder;
use effects::pedal_melody::PedalMelody;
use midi_devices::HAND_SONIC;
use midi_devices::*;
use midi_notes::*;
use patch::Patch;
use std::time::Duration;
use utils::add;
use utils::Boxable;
use utils::midi_filter::MidiFilter;

static PEDAL_NOTE: &str = "PEDAL_NOTE";
static BASE_NOTE: &str = "BASE_NOTE";

pub fn liebt_uns(_config: &Config) -> Patch {
    Patch::new_simple(
        "liebt uns",
        vec![
            PedalMelody::new(EXPRESS_PEDAL, PEDAL_NOTE, &[C5, 0, 1]).boxit(),
            ButtonMelodySustaining::new(
                PEDAL_NOTE,
                C5,
                1,
                BASE_NOTE,
                //                           add(vec![0, 7, 3, 8], A3),
                add(vec![12, 11, 7, 9, 12, 11, 0, 5, 12, 11, 7], C5),
                Duration::from_secs(4),
                Duration::from_millis(250),
            ).boxit(),
            MidiForwarder::new(MidiFilter::notes(BASE_NOTE), THROUGH_PORT).boxit(),
            HarmonyButtonMelody {
                harmony_input_filter: MidiFilter::notes_on(BASE_NOTE),
                stop_signal_filter: Some(MidiFilter::note(PEDAL_NOTE, 1)),
                button_melodies: vec![
                    ButtonMelody::new(
                        HAND_SONIC,
                        vec![74],
                        USB_MIDI_ADAPTER,
                        vec![0],
                        A4 as i8,
                        Duration::from_secs(2),
                    ),
                    ButtonMelody::new(
                        HAND_SONIC,
                        vec![60],
                        USB_MIDI_ADAPTER,
                        vec![12],
                        A4 as i8,
                        Duration::from_secs(2),
                    ),
                    ButtonMelody::new(
                        HAND_SONIC,
                        vec![64],
                        USB_MIDI_ADAPTER,
                        vec![7],
                        A4 as i8,
                        Duration::from_secs(2),
                    ),
                ],
                active: false,
            }.boxit()
        ],
        18, // 33
    )
}
