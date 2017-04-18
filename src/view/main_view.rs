extern crate piston_window;

use self::piston_window::*;
use std::sync::{Arc, Mutex};
use std::thread;
use chan::{Sender, Receiver};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ToViewEvents {
    BEAT(u8)
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum FromViewEvents {
    END
}

pub fn start_view(from_view_tx: Sender<FromViewEvents>, to_view_rx: Receiver<ToViewEvents>) {
    thread::spawn(move || {
        let mut window: PistonWindow =
            WindowSettings::new("Rust Midi Patcher", [1600, 900])
                .vsync(true)
                .fullscreen(false)
                .exit_on_esc(true).build().unwrap();

        let color_mutex = Arc::new(Mutex::new([1.0, 0.0, 0.0, 1.0]));
        let quadrant_mutex = Arc::new(Mutex::new(0));

        let color_mutex_thread = color_mutex.clone();
        let quadrant_mutex_thread = quadrant_mutex.clone();
        thread::spawn(move || {
            loop {
                chan_select! {
                    to_view_rx.recv() -> to_view_event => {
                        if let Some(ToViewEvents::BEAT(quadrant)) =  to_view_event {
                            let mut quadrant_mutex = quadrant_mutex_thread.lock().unwrap();
                            let mut color = color_mutex_thread.lock().unwrap();
                            *quadrant_mutex = quadrant;
                            let new_color = match quadrant {
                                0 => [1.0, 1.0, 0.0],
                                1 => [1.0, 0.0, 0.0],
                                2 => [0.0, 1.0, 0.0],
                                3 => [0.0, 0.0, 1.0],
                                _ => [0.0, 0.0, 0.0]
                            };
                            color[0] = new_color[0];
                            color[1] = new_color[1];
                            color[2] = new_color[2];
                            color[3] = 1.1;
                        }
                    }
                }
            }
        });

        while let Some(e) = window.next() {
            match e {
                Input::Render(r) => {
                    window.draw_2d(&e, |c, g| {
                        let quadrant: i32 = *quadrant_mutex.lock().unwrap() as i32;
                        let color: [f32; 4] = *color_mutex.lock().unwrap();
                        clear([0.0, 0.0, 0.0, 0.0], g);
                        let zoom = color[3] as f64;
                        let quadrant_width = r.width as f64 / 2.0;
                        let quadrant_height = r.height as f64 / 2.0;
                        let zoomed_width = quadrant_width * zoom;
                        let zoomed_height = quadrant_height * zoom;
                        let rect = [
                            ((quadrant % 2) as f64) * quadrant_width + ((quadrant_width - zoomed_width) / 2.0),
                            (quadrant / 2) as f64 * quadrant_height + ((quadrant_height - zoomed_height) / 2.0),
                            zoomed_width, zoomed_height
                        ];
                        rectangle(color, rect, c.transform, g);
                    });
                }
                Input::Update(_) => {
                    let mut color = color_mutex.lock().unwrap();
                    color[3] -= 0.02;
                }
                _ => {}
            }
        }
        from_view_tx.send(FromViewEvents::END);
    });
}