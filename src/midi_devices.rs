#![allow(dead_code)]

pub const THROUGH_PORT: &'static str = "Through";
pub const VMPK: &'static str = "VMPK";
pub const USB_MIDI_ADAPTER: &'static str = "USB";
pub const SAMPLE_PAD: &'static str = "SamplePad";
pub const MICRO_KEY: &'static str = "micro";
pub const STEP12: &'static str = "12Step";

pub const DEFAULT_IN_DEVICE: &'static str = USB_MIDI_ADAPTER;
pub const DEFAULT_OUT_DEVICE: &'static str = USB_MIDI_ADAPTER;

//pub const DEFAULT_IN_DEVICE: &'static str = VMPK;
//pub const DEFAULT_OUT_DEVICE: &'static str = THROUGH_PORT;