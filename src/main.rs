use sdl2::pixels::Color;

use crate::{
    processor::{Processor, load_rom},
    window::SDL,
};
pub mod processor;
pub mod window;

fn main() {
    let mut sdl = SDL::new(800, 800, "emulator".to_string());
    let mut canvas = sdl.window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut processor = Processor::new();

    load_rom(&mut processor);

    'running: loop {
        for event in sdl.event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'running,
                _ => {}
            }
        }
    }

    println!("Hello, world!");
}
