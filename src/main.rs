#[macro_use]
extern crate chan;
extern crate portmidi as pm;
extern crate chan_signal;


mod patch;
mod trigger;
mod effect;
mod midi_devices;
mod absolute_sleep;
mod utils;

mod songs {
    pub mod amazon;
}

use chan_signal::Signal;
use std::time::Duration;
use std::thread;
use pm::{PortMidi};


use songs::amazon::create_amazon;


fn print_devices(pm: &PortMidi) {
    for dev in pm.devices().unwrap() {
        println!("{}", dev);
    }
}


fn main() {
    println!("Started");
    let context = pm::PortMidi::new().unwrap();
    print_devices(&context);


    let mut patch = create_amazon(&context);

    const BUF_LEN: usize = 1024;
    let os_signal = chan_signal::notify(&[Signal::INT, Signal::TERM]);
    let (tx, rx) = chan::sync(0);

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

    loop {
        chan_select! {
            rx.recv() -> midi_events => {
                let (device, events) = midi_events.unwrap();
                for event in events {
                    if event.message.status == 248 {
                        continue
                    }
                    patch.on_midi_event(&device, event.message);
                }
            },
            os_signal.recv() -> os_sig => {
                println!("received signal: {:?}", os_sig);
                if os_sig == Some(Signal::INT) {
                    break;
                }
            }
        }
    }
}