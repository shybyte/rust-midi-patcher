use config::Config;
use effects::control_forwarder::ControlForwarder;
use effects::harmony_drum::HarmonyDrum;
use effects::pedal_melody::PedalMelody;
use effects::button_melody::ButtonMelody;
use microkorg::*;
use midi_devices::*;
use midi_notes::*;
use patch::Patch;
use std::time::Duration;
use trigger::Trigger;


pub fn liebeslieder(_config: &Config) -> Patch {
    Patch::new("liebeslieder", vec![
        (
            Box::new(Trigger::never()),
            Box::new(PedalMelody::new(
                EXPRESS_PEDAL, "LOOP", &[C5]))
        ),
        (
            Box::new(Trigger::never()),
            Box::new(ButtonMelody::new(
                "LOOP", C5, USB_MIDI_ADAPTER,
                add(vec![0, 4, -1, -3, 0, 4, 7, 5], C5 as i16),
                Duration::from_secs(2)))
        ),
    ], 45, None)
}

pub fn sicherheitskopie(_config: &Config) -> Patch {
    Patch::new("sicherheitskopie", vec![
        (
            Box::new(Trigger::never()),
            Box::new(PedalMelody::new(
                EXPRESS_PEDAL, "LOOP", &[C5]))
        ),
        (
            Box::new(Trigger::new("LOOP", C5)),
            Box::new(HarmonyDrum::new(
                USB_MIDI_ADAPTER, USB_MIDI_ADAPTER, (C2, C7),
                vec![5, 3, 1, 5, 3, 1, 8, 7, 5],
                Duration::from_millis(100), Duration::from_millis(200), Duration::from_secs(2)))
        ),
    ], 45, None)
}


pub fn young(_config: &Config) -> Patch {
    Patch::new("Young", vec![
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
    ], 45, None)
}


pub fn diktator(_config: &Config) -> Patch {
    Patch::new("diktator", vec![
        (
            Box::new(Trigger::never()),
            Box::new(ControlForwarder::new(
                EXPRESS_PEDAL, USB_MIDI_ADAPTER, CUTOFF))
        ),
    ], 45, None)
}


pub fn liebt_uns(_config: &Config) -> Patch {
    Patch::new("liebt uns", vec![
        (
            Box::new(Trigger::never()),
            Box::new(PedalMelody::new(
                EXPRESS_PEDAL, "LOOP", &[C5]))
        ),
        (
            Box::new(Trigger::new("LOOP", C5)),
            Box::new(HarmonyDrum::new(
                USB_MIDI_ADAPTER, USB_MIDI_ADAPTER, (C2, C5), vec![4, 7, 4, 0, 4, 7, 16, 12], Duration::from_millis(100), Duration::from_millis(200), Duration::from_secs(2)))
        ),
    ], 45, None)
}


pub fn add(xs: Vec<i16>, y: i16) -> Vec<u8> {
    xs.iter().map(|x| (x + y) as u8).collect()
}
