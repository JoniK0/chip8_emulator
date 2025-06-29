pub struct Processor {
    pub pc: usize,
    pub sp: usize,
    pub memory: [u8; 4000],
    pub v_register: [u8; 16],
    pub i_register: u16,
}

impl Processor {
    pub fn new() -> Self {
        let memory = [0; 4000];
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
