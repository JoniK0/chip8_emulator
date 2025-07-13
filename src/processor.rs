use crate::window::SDL;
use std::time::Duration;

use sdl2::{EventPump, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, sys::False};

pub const HEIGHT: u32 = 512;
pub const WIDTH: u32 = 1024;
pub const PIXEL: u32 = WIDTH / 64;

const FONTSIZE: usize = 80;
pub const FONT: [u8; FONTSIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Processor {
    pub pc: u16,
    sp: usize,
    memory: [u8; 4096],
    pub display: [u8; 32 * 64],
    stack: [u16; 64],
    v_register: [u8; 16],
    i_register: u16,
    delay_timer: u8,
    pub keys: [bool; 16],
}

impl Processor {
    pub fn new() -> Self {
        let memory = [0; 4096];
        let display = [0; 32 * 64];
        let stack = [0; 64];
        let pc = 512;
        let sp = 0;
        let v_register = [0; 16];
        let i_register = 0;
        let delay_timer = 0;
        let keys = [false; 16];

        Processor {
            pc,
            sp,
            memory,
            display,
            stack,
            v_register,
            i_register,
            delay_timer,
            keys,
        }
    }
}

pub fn load_rom(processor: &mut Processor) -> Vec<Vec<char>> {
    let bytes = std::fs::read("./src/Pong.ch8").unwrap();

    let mut n = 0;
    for byte in bytes.clone() {
        processor.memory[512 + n] = byte;
        n += 1;
    }

    processor.memory[0..FONTSIZE].copy_from_slice(&FONT);

    let hex_strings = bytes
        .into_iter()
        .map(|x| format!("{:02x}", x))
        .collect::<Vec<String>>();
    let four_tuples = hex_strings
        .chunks(2)
        .into_iter()
        .map(|list| {
            list.iter()
                .map(|x| x.chars().collect::<Vec<char>>())
                .flatten()
                .collect()
        })
        .collect::<Vec<Vec<char>>>();

    return four_tuples;
}

fn sleep(processor: &mut Processor) -> () {
    let dt = processor.delay_timer as u64;
    if dt > 0 {
        std::thread::sleep(Duration::from_millis(dt * 100 / 6));
    }
    processor.delay_timer = 0;
}

pub fn execute(
    canvas: &mut Canvas<sdl2::video::Window>,
    processor: &mut Processor,
    instruction: &Vec<char>,
) -> () {
    sleep(processor);
    match instruction.as_slice() {
        ['0', n2, n3, n4] => match_0(processor, (n2, n3, n4)),
        ['1', n2, n3, n4] => jump(processor, n2, n3, n4),
        ['2', n2, n3, n4] => call(processor, n2, n3, n4),
        ['3', n2, n3, n4] => skip_eqaul(processor, n2, n3, n4),
        ['4', n2, n3, n4] => skip_not_eqaul(processor, n2, n3, n4),
        ['5', n2, n3, '0'] => skip_eqaul_registers(processor, n2, n3),
        ['6', n2, n3, n4] => load_byte_to_register(processor, n2, n3, n4),
        ['7', n2, n3, n4] => add_byte_to_register(processor, n2, n3, n4),
        ['8', n2, n3, n4] => match_8(processor, (n2, n3, n4)),
        ['9', n2, n3, '0'] => skip_not_eqaul_registers(processor, n2, n3),
        ['a', n2, n3, n4] => load_i_register(processor, n2, n3, n4),
        ['b', n2, n3, n4] => jump_addv0(processor, n2, n3, n4),
        ['c', n2, n3, n4] => random(processor, n2, n3, n4),
        ['d', n2, n3, n4] => draw(processor, canvas, n2, n3, n4),
        ['e', n2, '9', 'e'] => skip_next_if_key_pressed(processor, n2),
        ['e', n2, 'a', '1'] => skip_next_if_key_not_pressed(processor, n2),
        ['f', n2, n3, n4] => match_f(processor, (n2, n3, n4)),
        [n1, n2, n3, n4] => println!("n1: {:?}, n2: {:?}, n3: {:?}, n4: {:?}", n1, n2, n3, n4),
        other => println!(
            "Expected to receive four elements but received {:?} instead",
            other
        ),
    };
    processor.pc += 2;
}

pub fn key_code_to_hex(keycode: sdl2::keyboard::Keycode) -> Option<usize> {
    match keycode {
        sdl2::keyboard::Keycode::Num1 => Some(0x01),
        sdl2::keyboard::Keycode::Num2 => Some(0x02),
        sdl2::keyboard::Keycode::Num3 => Some(0x03),
        sdl2::keyboard::Keycode::Num4 => Some(0x0c),
        sdl2::keyboard::Keycode::Q => Some(0x04),
        sdl2::keyboard::Keycode::W => Some(0x05),
        sdl2::keyboard::Keycode::E => Some(0x06),
        sdl2::keyboard::Keycode::R => Some(0x0d),
        sdl2::keyboard::Keycode::A => Some(0x07),
        sdl2::keyboard::Keycode::S => Some(0x08),
        sdl2::keyboard::Keycode::D => Some(0x09),
        sdl2::keyboard::Keycode::F => Some(0x0e),
        sdl2::keyboard::Keycode::Y => Some(0x0A),
        sdl2::keyboard::Keycode::X => Some(0x00),
        sdl2::keyboard::Keycode::C => Some(0x0B),
        sdl2::keyboard::Keycode::V => Some(0x0F),
        _ => None,
    }
}

fn skip_next_if_key_pressed(processor: &mut Processor, n2: &char) {
    let key = processor.v_register[n2.to_digit(16).unwrap() as usize] as usize;
    if processor.keys[key] {
        processor.pc += 2;
    }
}

fn skip_next_if_key_not_pressed(processor: &mut Processor, n2: &char) {
    let key = processor.v_register[n2.to_digit(16).unwrap() as usize] as usize;
    if processor.keys[key] {
        return;
    } else {
        processor.pc += 2;
    }
}

fn match_0(processor: &mut Processor, elements: (&char, &char, &char)) {
    match elements {
        ('0', 'e', '0') => clear_screen(processor),
        ('0', 'e', 'e') => ret(processor),
        _otherwise => println!(
            "This instruction is only used on the old computers on which Chip-8 was originally implemented."
        ),
    }
}

fn match_8(processor: &mut Processor, elements: (&char, &char, &char)) {
    match elements {
        (n1, n2, '0') => load_register_to_register(processor, n1, n2),
        (n1, n2, '1') => bitwise_or(processor, n1, n2),
        (n1, n2, '2') => bitwise_and(processor, n1, n2),
        (n1, n2, '3') => bitwise_xor(processor, n1, n2),
        (n1, n2, '4') => add_carryflag(processor, n1, n2),
        (n1, n2, '5') => subtract_registers(processor, n1, n2),
        (n1, _n2, '6') => rightshift_register(processor, n1),
        (n1, n2, '7') => subtract_n_registers(processor, n1, n2),
        (n1, _n2, 'e') => leftshift_register(processor, n1),
        _otherwise => println!("nothing matched to 8: instruction -> 8 {:?}", elements),
    }
}

fn match_f(processor: &mut Processor, elements: (&char, &char, &char)) {
    match elements {
        (n2, '0', '7') => load_delay_timer(processor, n2),
        (_n2, '0', 'a') => (), //println!("f: {:?}", elements),
        (n2, '1', '5') => set_delay_timer(processor, n2),
        (_n2, '1', '8') => (), // println!("f: {:?}", elements),
        (n2, '1', 'e') => add_v_to_i_register(processor, n2),
        (n2, '2', '9') => load_hex_sprite(processor, n2),
        (n2, '3', '3') => store_bcd(processor, n2),
        (n2, '5', '5') => load_registers_to_memory(processor, n2),
        (n2, '6', '5') => read_memory_to_register(processor, n2),
        _ => (),
    }
}

fn store_bcd(processor: &mut Processor, n2: &char) {
    let bcd = processor.v_register[n2.to_digit(16).unwrap() as usize];
    let hundreds = bcd / 100;
    let tens = (bcd - hundreds * 100) / 10;
    let ones = bcd - (hundreds * 100) - (tens * 10);
    let index = processor.i_register as usize;
    processor.memory[index] = hundreds;
    processor.memory[index + 1] = tens;
    processor.memory[index + 2] = ones;
}

fn load_registers_to_memory(processor: &mut Processor, n2: &char) {
    for n in 0..=n2.to_digit(16).unwrap() as usize {
        processor.memory[processor.i_register as usize + n] = processor.v_register[n];
    }
}

fn read_memory_to_register(processor: &mut Processor, n2: &char) {
    for n in 0..=n2.to_digit(16).unwrap() as usize {
        processor.v_register[n] = processor.memory[processor.i_register as usize + n];
    }
}

fn clear_screen(processor: &mut Processor) {
    processor.display = [0; 32 * 64];
}

fn ret(processor: &mut Processor) {
    processor.pc = processor.stack[processor.sp] as u16;
    processor.sp -= 1;
}
fn jump(processor: &mut Processor, n2: &char, n3: &char, n4: &char) {
    let mut str = String::new();
    str.push(*n2);
    str.push(*n3);
    str.push(*n4);
    let address = u16::from_str_radix(&str, 16).unwrap();
    processor.pc = address - 2;
}

fn call(processor: &mut Processor, n2: &char, n3: &char, n4: &char) {
    processor.sp += 1;
    processor.stack[processor.sp] = processor.pc;

    processor.pc = parse_3chars(n2, n3, n4) - 2;
}

fn skip_eqaul(processor: &mut Processor, n2: &char, n3: &char, n4: &char) {
    if processor.v_register[n2.to_digit(16).unwrap() as usize] == parse_2chars(n3, n4) as u8 {
        processor.pc += 2;
    }
}
fn skip_not_eqaul(processor: &mut Processor, n2: &char, n3: &char, n4: &char) {
    if processor.v_register[n2.to_digit(16).unwrap() as usize] != parse_2chars(n3, n4) as u8 {
        processor.pc += 2;
    }
}

fn skip_eqaul_registers(processor: &mut Processor, n2: &char, n3: &char) {
    if processor.v_register[n2.to_digit(16).unwrap() as usize]
        == processor.v_register[n3.to_digit(16).unwrap() as usize]
    {
        processor.pc += 2;
    }
}

fn skip_not_eqaul_registers(processor: &mut Processor, n2: &char, n3: &char) {
    if processor.v_register[n2.to_digit(16).unwrap() as usize]
        != processor.v_register[n3.to_digit(16).unwrap() as usize]
    {
        processor.pc += 2;
    }
}

fn load_byte_to_register(processor: &mut Processor, n2: &char, n3: &char, n4: &char) {
    processor.v_register[n2.to_digit(16).unwrap() as usize] = parse_2chars(n3, n4) as u8;
}

fn load_register_to_register(processor: &mut Processor, n1: &char, n2: &char) {
    processor.v_register[n1.to_digit(16).unwrap() as usize] =
        processor.v_register[n2.to_digit(16).unwrap() as usize];
}

fn add_byte_to_register(processor: &mut Processor, n2: &char, n3: &char, n4: &char) {
    processor.v_register[n2.to_digit(16).unwrap() as usize] += parse_2chars(n3, n4) as u8;
}

fn bitwise_or(processor: &mut Processor, n1: &char, n2: &char) {
    processor.v_register[n1.to_digit(16).unwrap() as usize] = processor.v_register
        [n1.to_digit(16).unwrap() as usize]
        | processor.v_register[n2.to_digit(16).unwrap() as usize]
}

fn bitwise_and(processor: &mut Processor, n1: &char, n2: &char) {
    processor.v_register[n1.to_digit(16).unwrap() as usize] &=
        processor.v_register[n2.to_digit(16).unwrap() as usize]
}

fn bitwise_xor(processor: &mut Processor, n1: &char, n2: &char) {
    processor.v_register[n1.to_digit(16).unwrap() as usize] ^=
        processor.v_register[n2.to_digit(16).unwrap() as usize]
}

fn add_carryflag(processor: &mut Processor, n1: &char, n2: &char) {
    let overflow: u32 = processor.v_register[n1.to_digit(16).unwrap() as usize] as u32
        + processor.v_register[n2.to_digit(16).unwrap() as usize] as u32;
    if overflow > 255 {
        processor.v_register[15] = 1;
    } else {
        processor.v_register[15] = 0;
    }
    processor.v_register[n1.to_digit(16).unwrap() as usize] +=
        processor.v_register[n2.to_digit(16).unwrap() as usize];
}

fn subtract_registers(processor: &mut Processor, n1: &char, n2: &char) {
    if processor.v_register[n1.to_digit(16).unwrap() as usize]
        > processor.v_register[n2.to_digit(16).unwrap() as usize]
    {
        processor.v_register[15] = 1;
    } else {
        processor.v_register[15] = 0;
    }
    processor.v_register[n1.to_digit(16).unwrap() as usize] -=
        processor.v_register[n2.to_digit(16).unwrap() as usize]; // note: what happens when n2 > n1?
}

fn rightshift_register(processor: &mut Processor, n1: &char) {
    let least_sig_bit = processor.v_register[n1.to_digit(16).unwrap() as usize] & 1;
    processor.v_register[15] = least_sig_bit;

    processor.v_register[n1.to_digit(16).unwrap() as usize] /= 2;
}

fn subtract_n_registers(processor: &mut Processor, n1: &char, n2: &char) {
    if processor.v_register[n2.to_digit(16).unwrap() as usize]
        > processor.v_register[n1.to_digit(16).unwrap() as usize]
    {
        processor.v_register[15] = 1;
    } else {
        processor.v_register[15] = 0;
    }
    processor.v_register[n1.to_digit(16).unwrap() as usize] = processor.v_register
        [n2.to_digit(16).unwrap() as usize]
        - processor.v_register[n1.to_digit(16).unwrap() as usize]; // note: what happens when n1 > n2?
}

fn leftshift_register(processor: &mut Processor, n1: &char) {
    let most_sig_bit = processor.v_register[n1.to_digit(16).unwrap() as usize] & 128;
    processor.v_register[15] = most_sig_bit;
    processor.v_register[n1.to_digit(16).unwrap() as usize] /= 2;
}

fn load_i_register(processor: &mut Processor, n2: &char, n3: &char, n4: &char) {
    processor.i_register = parse_3chars(n2, n3, n4);
}

fn jump_addv0(processor: &mut Processor, n2: &char, n3: &char, n4: &char) {
    processor.pc = parse_3chars(n2, n3, n4) + processor.v_register[0] as u16;
}

fn random(processor: &mut Processor, n2: &char, n3: &char, n4: &char) {
    processor.v_register[n2.to_digit(16).unwrap() as usize] =
        rand::random_range(0..255) & parse_2chars(n3, n4) as u8;
}

fn load_hex_sprite(processor: &mut Processor, n2: &char) {
    processor.i_register = 5 * processor.v_register[n2.to_digit(16).unwrap() as usize] as u16;
}

fn set_delay_timer(processor: &mut Processor, n2: &char) {
    processor.delay_timer = processor.v_register[n2.to_digit(16).unwrap() as usize];
}

fn load_delay_timer(processor: &mut Processor, n2: &char) {
    processor.v_register[n2.to_digit(16).unwrap() as usize] = processor.delay_timer;
}

fn draw(
    processor: &mut Processor,
    canvas: &mut Canvas<sdl2::video::Window>,
    n1: &char,
    n2: &char,
    n3: &char,
) {
    let mut y_pos;

    for n in 0..n3.to_digit(16).unwrap() as usize {
        let x = processor.v_register[n1.to_digit(16).unwrap() as usize] as usize;
        let y = processor.v_register[n2.to_digit(16).unwrap() as usize] as usize;

        for b in 0..8 as usize {
            let i_register = processor.i_register as usize;
            // TODO: create constants for the number of pixels on the horizontal
            // and vertiacal axis and use these instead of hard coded numbers 32 and 64
            //let y_pos = ((n + y) * 32) % 32;
            y_pos = (y + n) % 32;
            let x_pos = (x + b) % 64;

            processor.v_register[15] = 0;
            let index = (x_pos + (y_pos * 64)) % 2048;
            //let index = std::cmp::min(x_pos + (y_pos * 64) + b, 2047);
            if processor.display[index] == 1 && processor.memory[i_register + n] == 1 {
                processor.v_register[15] = 1;
            }

            processor.display[index] ^= (processor.memory[i_register + n] >> (7 - b)) & 1;
            draw_pixel(canvas, processor, index);
        }

        //drawscreen(canvas, processor);
    }
}

fn draw_pixel(canvas: &mut Canvas<sdl2::video::Window>, processor: &mut Processor, index: usize) {
    let color = processor.display[index];
    canvas.set_draw_color(Color::RGB(255 * color, 255 * color, 255 * color));
    let x = (index as u32 * PIXEL) % WIDTH;
    let y = (index as u32 * PIXEL) / WIDTH * PIXEL;
    let _ = canvas.fill_rect(Rect::new(x as i32, y as i32, PIXEL, PIXEL));
    canvas.present();
}

pub fn drawscreen(canvas: &mut Canvas<sdl2::video::Window>, processor: &mut Processor) {
    let pixels = 64 * 32;
    for n in 0..pixels {
        //let color = 1;
        let color = processor.display[n as usize];
        canvas.set_draw_color(Color::RGB(255 * color, 255 * color, 255 * color));
        let x = (n * PIXEL) % WIDTH;
        let y = (n * PIXEL) / WIDTH * PIXEL;
        let _ = canvas.fill_rect(Rect::new(x as i32, y as i32, PIXEL, PIXEL));
        canvas.present();
    }
}

fn add_v_to_i_register(processor: &mut Processor, n2: &char) {
    processor.i_register += processor.v_register[n2.to_digit(16).unwrap() as usize] as u16;
}

/*
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
*/

fn parse_3chars(c1: &char, c2: &char, c3: &char) -> u16 {
    // NOTE: this function doesn't work. The number to parse are in hex
    let mut str = String::new();
    str.push(*c1);
    str.push(*c2);
    str.push(*c3);
    let num = u16::from_str_radix(&str, 16).unwrap();
    return num;
}
fn parse_2chars(c1: &char, c2: &char) -> u16 {
    let mut str = String::new();
    str.push(*c1);
    str.push(*c2);
    let num = u16::from_str_radix(&str, 16).unwrap();
    return num;
}
pub fn decode() -> () {}
