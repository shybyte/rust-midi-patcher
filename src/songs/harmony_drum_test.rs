use std::time::Duration;
use patch::Patch;
use trigger::Trigger;
use effects::harmony_drum::HarmonyDrum;
use config::Config;
use midi_notes::*;
use midi_devices::*;


pub fn create_harmony_drum_test_song(_config: &Config) -> Patch {
    Patch::new("HarmonyTest", vec![
        //        (Box::new(Trigger::new(&config.default_in_device, 43)), Box::new(note_seq(chorus_notes.clone()))),
        //        (Box::new(Trigger::new(SAMPLE_PAD, 45)), Box::new(note_seq(vec![]))),
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
//        (
//            Box::new(Trigger::new(SAMPLE_PAD, 38)),
//            Box::new(HarmonyDrum::new(
//                USB_MIDI_ADAPTER, USB_MIDI_ADAPTER, (C2, C5), vec![4, 7, 4, 0, 4, 7, 16, 12], Duration::from_millis(100), Duration::from_millis(200), Duration::from_secs(2)))
//        ),
//        (
//            Box::new(Trigger::new(KBOARD, 48)),
//            Box::new(HarmonyDrum::new(KBOARD, THROUGH_PORT, (D4, C5), vec![7, 12, 19]))
//        ),
    ], 45, None)
}
