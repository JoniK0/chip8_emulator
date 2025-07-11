use std::u8;

use sdl2::{pixels::Color, rect::Rect, render::Canvas, sys::Window};

use crate::{
    processor::{Processor, load_rom},
    window::SDL,
};
pub mod processor;
pub mod window;

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 512;
const PIXEL: u32 = WIDTH / 64;

fn main() {
    let mut sdl = SDL::new(WIDTH, HEIGHT, "emulator".to_string());
    let mut canvas = sdl.window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    canvas.set_draw_color(Color::RGB(255, 255, 255));

    canvas.fill_rect(Rect::new(300, 300, PIXEL, PIXEL));
    canvas.present();

    drawbyte(&mut canvas, 138, 25, 30);

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

fn drawbyte(canvas: &mut Canvas<sdl2::video::Window>, byte: u8, x: u32, y: u32) {
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    for n in 0..9 {
        let pixel = (byte >> n) & 1;
        canvas.set_draw_color(Color::RGB(255 * pixel, 255 * pixel, 255 * pixel));
        canvas.fill_rect(Rect::new(
            ((x * PIXEL + PIXEL * 8) - (PIXEL * n)) as i32,
            (y * PIXEL) as i32,
            PIXEL,
            PIXEL,
        ));
        canvas.present();
    }
}
