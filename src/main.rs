use std::{thread::sleep, time::Duration};

use sdl2::pixels::Color;

use crate::{
    processor::{Processor, key_code_to_hex, load_rom},
    window::SDL,
};
pub mod processor;
pub mod window;

fn main() {
    let mut sdl = SDL::new(processor::WIDTH, processor::HEIGHT, "emulator".to_string());
    let mut canvas = sdl.window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    canvas.set_draw_color(Color::RGB(255, 255, 255));

    canvas.present();

    let mut processor = Processor::new();

    let program = load_rom("./src/Tron.ch8", &mut processor);

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
        let instr = &program[(program_counter - 512) / 2];
        //println!("instruction: {:?}", instr);
        processor::execute(
            &mut canvas,
            &mut processor,
            &program[(program_counter - 512) / 2],
        );
    }
}
