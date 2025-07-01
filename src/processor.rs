pub struct Processor {
    pc: u16,
    sp: u16,
    memory: [u8; 4096],
    v_register: [u8; 16],
    i_register: u16,
}

impl Processor {
    pub fn new() -> Self {
        let memory = [0; 4096];
        let pc = 0;
        let sp = 0;
        let v_register = [0; 16];
        let i_register = 0;

        Processor {
            pc,
            sp,
            memory,
            v_register,
            i_register,
        }
    }
}

pub fn load_rom() -> () {
    let bytes = std::fs::read("./src/Pong.ch8").unwrap();

    let new_vec = bytes
        .into_iter()
        .map(|x| format!("{:02x}", x))
        .collect::<Vec<String>>();
    let new_vec2 = new_vec.chunks(2).collect::<Vec<&[String]>>();
    let new_vec3 = new_vec2.iter().map(|list| {
        list.iter()
            .map(|x| x.chars().map(|y| y.to_string()).collect::<Vec<String>>())
            .flatten()
            .into_iter()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<String>>()
    });

    for byte in new_vec3 {
        println!("{:?}", byte);
    }
    // for byte in bytes.iter() {
    //     println!("{:02x}", byte);
    // }
    //
}

pub fn decode() -> () {}
