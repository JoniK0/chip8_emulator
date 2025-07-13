use std::{thread::sleep, time::Duration};

use sdl2::pixels::Color;

use crate::{
    processor::{Processor, key_code_to_hex, load_rom},
    window::SDL,
};
pub mod processor;
pub mod window;

pub const WIDTH: u32 = 1024;
pub const HEIGHT: u32 = 512;
pub const PIXEL: u32 = WIDTH / 64;

fn main() {
    let mut sdl = SDL::new(WIDTH, HEIGHT, "emulator".to_string());
    let mut canvas = sdl.window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    canvas.set_draw_color(Color::RGB(255, 255, 255));

    canvas.present();

    let mut processor = Processor::new();

    let program = load_rom(&mut processor);

    'running: loop {
        for event in sdl.event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'running,
                sdl2::event::Event::KeyDown {
                    keycode: Some(code),
                    ..
                } => match key_code_to_hex(code) {
                    Some(hex) => {
                        processor.keys[hex] = true;
                    }
                    None => (),
                },
                sdl2::event::Event::KeyUp {
                    keycode: Some(code),
                    ..
                } => match key_code_to_hex(code) {
                    Some(hex) => {
                        processor.keys[hex] = false;
                    }
                    None => (),
                },
                _ => {}
            }
        }

        let program_counter = processor.pc as usize;
        sleep(Duration::from_millis(1));
        processor::execute(
            &mut canvas,
            &mut processor,
            &program[(program_counter - 512) / 2],
        );
    }
}
