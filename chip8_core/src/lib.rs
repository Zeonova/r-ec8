// Chip-8
const START_ADDR: u16 = 0x200;
const RAM_SIZE: usize = 4096;
const NUM_REGISTERS: usize = 16;
const STACK_SIZE: usize = 16;
// display setup
pub const SCREEN_W: usize = 64;
pub const SCREEN_H: usize = 32;
pub struct Emu {
    // program counter
    pc: u16,
    ram: [u8; RAM_SIZE],
    screen: [bool; SCREEN_H * SCREEN_W],

    v_reg: [u8; NUM_REGISTERS],
    i_reg: u16,

    // stack pointer
    sp: u16,
    stack: [u16; STACK_SIZE],

    // Delay Timer
    dt: u8,
    // Sound Timer
    st: u8,
}

impl Default for Emu {
    fn default() -> Self {
        Self {
            pc: START_ADDR,
            ram: [0; RAM_SIZE],
            screen: [false; SCREEN_H * SCREEN_W],
            v_reg: [0; NUM_REGISTERS],
            i_reg: 0,
            sp: 0,
            stack: [0; STACK_SIZE],
            dt: 0,
            st: 0,
        }
    }
}

impl Emu {
    fn push(&mut self, val: u16) {
        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }

    fn pop(&mut self) {
        self.sp -= 1;
        self.stack[self.sp as usize];
    }
}
