//use std::intrinsics::wrapping_add;

//use crate::processor;
use rand::Rng;
//use sdl2::render::Canvas;
use sdl2::{pixels::Color, rect::Rect, render::Canvas};

pub const HEIGHT: u32 = 512;
pub const WIDTH: u32 = 1024;
pub const PIXEL: u32 = WIDTH / 64;

//use crate::drawbyte;

pub struct Processor {
    pc: u16,
    sp: usize,
    memory: [u8; 4096],
    display: [u8; 32 * 64],
    stack: [u8; 64],
    v_register: [u8; 16],
    i_register: u16,
}

impl Processor {
    pub fn new() -> Self {
        let memory = [0; 4096];
        let display = [0; 32 * 64];
        let stack = [0; 64];
        let pc = 0;
        let sp = 0;
        let v_register = [0; 16];
        let i_register = 0;

        Processor {
            pc,
            sp,
            memory,
            display,
            stack,
            v_register,
            i_register,
        }
    }
}

pub fn load_rom(processor: &mut Processor, canvas: &mut Canvas<sdl2::video::Window>) -> () {
    let bytes = std::fs::read("./src/Pong.ch8").unwrap();

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

    for byte in four_tuples {
        match byte.as_slice() {
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
            [n1, n2, n3, n4] => println!("n1: {:?}, n2: {:?}, n3: {:?}, n4: {:?}", n1, n2, n3, n4),
            other => println!(
                "Expected to receive four elements but received {:?} instead",
                other
            ),
        };
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
        (n1, n2, '6') => rightshift_register(processor, n1),
        (n1, n2, '7') => subtractN_registers(processor, n1, n2),
        (n1, n2, 'e') => leftshift_register(processor, n1),
        _otherwise => println!("nothing matched to 8"),
    }
}

fn clear_screen(processor: &mut Processor) {
    //todo!("this function should clear the screen")
    processor.display = [0; 32 * 64];
}

fn ret(processor: &mut Processor) {
    //todo!(
    //    "The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer"
    //)
    processor.pc = processor.stack[processor.sp] as u16;
    processor.sp -= 1;
}
fn jump(processor: &mut Processor, n2: &char, n3: &char, n4: &char) {
    let mut str = String::new();
    str.push(*n2);
    str.push(*n3);
    str.push(*n4);
    let address = u16::from_str_radix(&str, 16).unwrap();
    //let address: u16 = str.parse().unwrap();
    processor.pc = address;
}

fn call(processor: &mut Processor, n2: &char, n3: &char, n4: &char) {
    processor.sp += 1;
    processor.stack[processor.sp] = processor.pc as u8; //note: pc as u8 only temporary solution
    processor.pc = parse_3chars(n2, n3, n4);
}

fn skip_eqaul(processor: &mut Processor, n2: &char, n3: &char, n4: &char) {
    if processor.v_register[n2.to_digit(16).unwrap() as usize] == parse_2chars(n3, n4) as u8 {
        processor.pc += 1;
    }
}
fn skip_not_eqaul(processor: &mut Processor, n2: &char, n3: &char, n4: &char) {
    if processor.v_register[n2.to_digit(16).unwrap() as usize] != parse_2chars(n3, n4) as u8 {
        processor.pc += 1;
    }
}

fn skip_eqaul_registers(processor: &mut Processor, n2: &char, n3: &char) {
    if processor.v_register[n2.to_digit(16).unwrap() as usize]
        == processor.v_register[n3.to_digit(16).unwrap() as usize]
    {
        processor.pc += 1;
    }
}

fn skip_not_eqaul_registers(processor: &mut Processor, n2: &char, n3: &char) {
    if processor.v_register[n2.to_digit(16).unwrap() as usize]
        != processor.v_register[n3.to_digit(16).unwrap() as usize]
    {
        processor.pc += 1;
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
    //print!(
    //    "adder: {:?}, {:?}",
    //    processor.v_register[n2.to_digit(16).unwrap() as usize],
    //    parse_2chars(n3, n4) as u8
    //);
    processor.v_register[n2.to_digit(16).unwrap() as usize] += parse_2chars(n3, n4) as u8; // note: unhandled overflow
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
    let overflow: u16 = processor.v_register[n1.to_digit(16).unwrap() as usize] as u16
        + processor.v_register[n2.to_digit(16).unwrap() as usize] as u16;
    if overflow > 255 {
        processor.v_register[15] = 1;
    } else {
        processor.v_register[15] = 0;
    }
    processor.v_register[n1.to_digit(16).unwrap() as usize] = overflow as u8;
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

fn subtractN_registers(processor: &mut Processor, n1: &char, n2: &char) {
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

fn draw(
    processor: &mut Processor,
    canvas: &mut Canvas<sdl2::video::Window>,
    n1: &char,
    n2: &char,
    n3: &char,
) {
    for n in 0..n3.to_digit(16).unwrap() as usize {
        let x = processor.v_register[n1.to_digit(16).unwrap() as usize] as usize;
        let y = processor.v_register[n2.to_digit(16).unwrap() as usize] as usize;

        for b in 0..9 as usize {
            let i_register = processor.i_register as usize;
            let y_pos = (n + y) * 64;
            let mut x_pos = x + b;
            if x_pos >= 64 {
                x_pos -= 64;
            }
            processor.v_register[15] = 0;
            if processor.display[x_pos + y_pos + b] == 1
                && processor.display[x_pos + y_pos + b] == 1
            {
                processor.v_register[15] = 1;
            }
            processor.display[x_pos + y_pos + b] ^= processor.memory[i_register + n];
        }

        drawscreen(processor, canvas);
    }
}

fn drawscreen(processor: &mut Processor, canvas: &mut Canvas<sdl2::video::Window>) {
    let pixels = WIDTH * HEIGHT;
    for n in 0..pixels {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.fill_rect(Rect::new(
            ((n % WIDTH) * PIXEL) as i32,
            ((n % HEIGHT) * PIXEL) as i32,
            PIXEL,
            PIXEL,
        ));
        canvas.present();
    }
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

fn parse_3chars(c1: &char, c2: &char, c3: &char) -> u16 {
    // note: this function doesnt work. the number to parse are in hex
    let mut str = String::new();
    str.push(*c1);
    str.push(*c2);
    str.push(*c3);
    //let num: u16 = str.parse().unwrap();
    let num = u16::from_str_radix(&str, 16).unwrap();
    return num;
}
fn parse_2chars(c1: &char, c2: &char) -> u16 {
    let mut str = String::new();
    str.push(*c1);
    str.push(*c2);
    //let num: u8 = str.parse().unwrap();
    let num = u16::from_str_radix(&str, 16).unwrap();
    return num;
}
pub fn decode() -> () {}
