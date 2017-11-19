use pm::{PortMidi, OutputPort, MidiMessage};

use midi2opc::midi_light_strip::{MidiLightStrip, MidiLightConfig};

pub use midi2opc::midi_light_strip::MidiLightPatch;

const BUF_LEN: usize = 1024;
const LED_COUNT: usize = 30;

pub struct VirtualMidiOutput {
    output_ports: Vec<OutputPort>,
    midi_light_strip: Option<MidiLightStrip>,
}

impl VirtualMidiOutput {
    pub fn new(context: &PortMidi) -> VirtualMidiOutput {
        let output_ports: Vec<OutputPort> = context.devices().unwrap().into_iter()
            .filter(|dev| dev.is_output())
            .map(|dev| context.output_port(dev, BUF_LEN).unwrap())
            .collect();
        let midi_light_strip = MidiLightStrip::start(MidiLightConfig {
            led_count: LED_COUNT,
            stream: false,
            flash: true,
            blink: true,
            max_note: 128,
            reversed: true,
        });
        VirtualMidiOutput { output_ports, midi_light_strip: midi_light_strip.ok() }
    }

    pub fn reconfigure(&mut self, midi_light_patch: &MidiLightPatch) {
        if let Some(ref midi_strip) = self.midi_light_strip {
            midi_strip.reconfigure(midi_light_patch);
        }
    }

    pub fn play(&mut self, output_name: &str, message: MidiMessage) {
        let output_port = self.output_ports.iter_mut()
            .find(|p| p.device().name().contains(output_name)).unwrap();
        output_port.write_message(message).ok();
    }

    pub fn play_and_visualize(&mut self, output_name: &str, message: MidiMessage) {
        self.play(output_name, message);
        self.visualize(message);
    }

    pub fn visualize(&mut self, message: MidiMessage) {
        if let Some(ref midi_strip) = self.midi_light_strip {
            midi_strip.on_raw_midi_message(message.status, message.data1, message.data2)
        }
    }

    pub fn stop(&mut self) {
        if let Some(ref midi_strip) = self.midi_light_strip {
            midi_strip.stop();
        }
        self.all_notes_off();
    }

    pub fn all_notes_off(&mut self) {
        let all_notes_off = MidiMessage {
            status: 176,
            data1: 123,
            data2: 0
        };
        for output_port in &mut self.output_ports {
            output_port.write_message(all_notes_off).ok();
        }
    }

}
