#[macro_use]
extern crate chan;
extern crate portmidi as pm;
extern crate chan_signal;
extern crate risp;
extern crate notify;


mod patch;
mod trigger;

mod effects {
    pub mod effect;
    pub mod note_sequencer;
    pub mod harmony_drum;
    pub mod sweep_down;
    pub mod control_sequencer;
}

mod config;
mod midi_devices;
mod load_patches;
mod absolute_sleep;
mod utils;
mod microkorg;
mod midi_notes;
mod watch;

mod songs {
    pub mod test;
    pub mod harmony_drum_test;
}

mod view {
    pub mod main_view;
}

use view::main_view::start_view;

use config::load_config;
use chan_signal::Signal;
use std::time::Duration;
use std::thread;
use std::sync::{Arc, Mutex};
use std::path::Path;
use config::Config;
use pm::{PortMidi};
use pm::{OutputPort};
use watch::*;
use patch::Patch;
use chan::{Sender};
use view::main_view::ToViewEvents;

use load_patches::*;


fn print_devices(pm: &PortMidi) {
    for dev in pm.devices().unwrap() {
        println!("{}", dev);
    }
}


fn main() {
    println!("Started");
    let config = load_config("config/config.risp").unwrap();

    let context = pm::PortMidi::new().unwrap();
    print_devices(&context);

    let output_ports: Vec<Arc<Mutex<OutputPort>>> = context.devices().unwrap().into_iter()
        .filter(|dev| dev.is_output())
        .map(|dev| Arc::new(Mutex::new(context.output_port(dev, BUF_LEN).unwrap())))
        .collect();

    let mut patches = load_patches(&config);
    let mut selected_patch = patches.iter().position(|p| p.name() == config.selected_patch).unwrap_or(0);

    const BUF_LEN: usize = 1024;

    let os_signal = chan_signal::notify(&[Signal::INT, Signal::TERM]);
    let tick = chan::tick_ms(50);
    let (_watch_tx, watch_rx) = watch_patches();
    let (tx, rx) = chan::sync(0);
    let (from_view_tx, from_view_rx) = chan::async();
    let (to_view_tx, to_view_rx) = chan::async();

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
                    tx.send((port.device(), events));
                }
            }
            thread::sleep(timeout);
        }
    });

    if config.view {
        start_view(from_view_tx, to_view_rx);
    }

    println!("Selected Patch = {:?}", patches[selected_patch].name());

    loop {
        chan_select! {
            tick.recv() -> _tick_events => {
                for file in get_changed_files(watch_rx.try_iter()) {
                    on_patch_file_change(&file, &config, &mut patches, &output_ports, &to_view_tx);
                }
            },
            rx.recv() -> midi_events => {
                let (device, events) = midi_events.unwrap();
                for event in events {
                    match event.message.status {
                        248 => continue,
                        192 => {
                            println!("program change {:?}", event.message);
                            let new_patch_i_option = patches.iter().position(|p|  p.program() == event.message.data1);
                            if let Some(new_patch_i) = new_patch_i_option {
                                patches[selected_patch].stop_running_effects();
                                selected_patch = new_patch_i;
                                println!("Selected Patch = {:?}", patches[selected_patch].name());
                            }

                        },
                        _ => {
                            println!("event = {:?}", event);
                            patches[selected_patch].on_midi_event(&output_ports, &device, event.message, &to_view_tx);
                        }
                    }
                }
            },
            os_signal.recv() -> os_sig => {
                println!("received os signal: {:?}", os_sig);
                if os_sig == Some(Signal::INT) {
                    break;
                }
            },
            from_view_rx.recv() -> _ => {
                break;
            }
        }
    }
}

fn on_patch_file_change(file: &Path, config: &Config, patches: &mut [Patch], output_ports: &[Arc<Mutex<OutputPort>>], to_view_tx: &Sender<ToViewEvents>) {
    println!("changed file = {:?}", file);
    match load_patch(file, config) {
        Ok(loaded_patch) => {
            println!("Loaded patch = {:?}", loaded_patch.name());
            if let Some(index) = patches.iter().position(|p| p.name() == loaded_patch.name()) {
                patches[index].update_from(loaded_patch, output_ports, to_view_tx);
            } else {
                println!("New Patch, ignore it for now...");
            }
        }
        Err(err) => println!("Error while loading changed patch: {:?}", err)
    }
}