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
            Box::new(HarmonyDrum::new(USB_MIDI_ADAPTER, USB_MIDI_ADAPTER, (C2, D3), vec![0]))
        ),
        (
            Box::new(Trigger::new(SAMPLE_PAD, 51)),
            Box::new(HarmonyDrum::new(USB_MIDI_ADAPTER, USB_MIDI_ADAPTER, (C2, D3), vec![0, 7, 12]))
        ),
    ], 0)
}
