#![allow(unused_mut)]

#[macro_use]
extern crate chan;
extern crate portmidi as pm;
extern crate chan_signal;
extern crate risp;
extern crate midi2opc;
extern crate notify;
extern crate midi_message;


mod patch;
mod trigger;

mod effects;

mod config;
mod carla_starter;
mod midi_devices;
mod midi_beat_tracker;
mod midi_event;
mod load_patches;
mod absolute_sleep;
mod utils;
mod microkorg;
mod midi_notes;
mod watch;
mod virtual_midi;

mod songs {
    pub mod test;
    pub mod harmony_drum_test;
    pub mod endstation_paradies;
    pub mod endstation;
}

use crate::config::load_config;
use chan_signal::Signal;
use std::time::Duration;
use std::thread;
use std::sync::{Arc, Mutex};
use std::path::Path;
use crate::config::Config;
use crate::pm::{PortMidi};
use crate::watch::*;
use crate::patch::Patch;
use crate::virtual_midi::VirtualMidiOutput;
use crate::carla_starter::CarlaStarter;

use crate::load_patches::*;


fn print_devices(pm: &PortMidi) {
    for dev in pm.devices().unwrap() {
        println!("{}", dev);
    }
}


fn main() {
    println!("Started");

    // Must run before any other thread starts.
    let os_signal = chan_signal::notify(&[Signal::INT, Signal::TERM]);

    let config = load_config("config/config.risp").unwrap();

    let context = pm::PortMidi::new().unwrap();
    print_devices(&context);

    let (tx, rx) = chan::sync(1000);
    let virtual_midi_output =  Arc::new(Mutex::new(VirtualMidiOutput::new(&context, tx.clone())));

    let mut patches = load_patches(&config);
    let mut selected_patch = patches.iter().position(|p| p.name() == config.selected_patch);
    let mut carla_starter = config.carla_path.clone().map(|it| CarlaStarter::new(&it));

    if let Some(sp) = selected_patch {
        patches[sp].init(&virtual_midi_output);
        if let Some(ref mut carla_starter) = carla_starter {
            carla_starter.on_patch_change(&patches[sp].name())
        }
    }

    const BUF_LEN: usize = 1024;


    let tick = chan::tick_ms(50);
    let (_watch_tx, watch_rx) = watch_patches();

    let in_devices: Vec<pm::DeviceInfo> = context.devices()
        .unwrap()
        .into_iter()
        .filter(|dev| dev.is_input())
        .collect();
    let in_ports: Vec<pm::InputPort> = in_devices.into_iter()
        .filter_map(|dev| {
            context.input_port(dev, BUF_LEN)
                .ok()
        })
        .collect();


    thread::spawn(move || {
        let timeout = Duration::from_millis(10);
        loop {
            for port in &in_ports {
                if let Ok(Some(events)) = port.read_n(BUF_LEN) {
                    for event in events.iter() {
                        tx.send((port.device().name().clone(), event.message));
                    }
                }
            }
            thread::sleep(timeout);
        }
    });

    println!("Selected Patch = {:?}", selected_patch.map( |p| patches[p].name()));

    loop {
        chan_select! {
            tick.recv() -> _tick_events => {
                for file in get_changed_files(watch_rx.try_iter()) {
                    on_patch_file_change(&file, &config, &mut patches, &virtual_midi_output);
                }
            },
            rx.recv() -> midi_events => {
                let (device, midi_message) = midi_events.unwrap();
                match midi_message.status {
                    248 => continue,
                    192 if config.patch_selection => {
                        println!("program change {:?}", midi_message);
                        let new_patch_i_option = patches.iter().position(|p|  p.program() == midi_message.data1);
                        if let Some(new_patch_i) = new_patch_i_option {
                            if let Some(sp) = selected_patch {
                                patches[sp].stop_running_effects(&virtual_midi_output);
                            }
                            selected_patch = new_patch_i_option;
                            let patch = &mut patches[new_patch_i];
                            patch.init(&virtual_midi_output);
                            println!("Selected Patch = {:?}", patch.name());

                            if let Some(ref mut carla_starter) = carla_starter {
                                carla_starter.on_patch_change(patch.name())
                            }
                        }

                    },
                    _ => {
                            println!("{:?} {:?} {:x}", device, midi_message, midi_message.status);
                        if let Some(sp) = selected_patch {
                            patches[sp].on_midi_event(&device, midi_message, &virtual_midi_output);
                        }
                        virtual_midi_output.lock().unwrap().visualize(midi_message);
                    }
                }
            },
            os_signal.recv() -> os_sig => {
                println!("received os signal: {:?}", os_sig);
                if os_sig == Some(Signal::INT) {
                    break;
                }
            }
        }
    }

    virtual_midi_output.lock().unwrap().stop();
}

fn on_patch_file_change(file: &Path, config: &Config, patches: &mut [Patch],
                        virtual_midi_out: &Arc<Mutex<VirtualMidiOutput>>) {
    println!("changed file = {:?}", file);
    match load_patch(file, config) {
        Ok(loaded_patch) => {
            println!("Loaded patch = {:?}", loaded_patch.name());
            if let Some(index) = patches.iter().position(|p| p.name() == loaded_patch.name()) {
                patches[index].update_from(loaded_patch, virtual_midi_out);
            } else {
                println!("New Patch, ignore it for now...");
            }
        }
        Err(err) => println!("Error while loading changed patch: {:?}", err)
    }
}
