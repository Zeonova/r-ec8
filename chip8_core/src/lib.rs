// Chip-8
const FONTSET_SIZE: usize = 80;
const FONTSET: [u8; FONTSET_SIZE] = [
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

// system setup
const START_ADDR: u16 = 0x200;
const RAM_SIZE: usize = 4096;
const NUM_REGISTERS: usize = 16;
const STACK_SIZE: usize = 16;
const NUM_KEYS: usize = 16;

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
    keys: [bool; NUM_KEYS],

    // Delay Timer
    dt: u8,
    // Sound Timer
    st: u8,
}

impl Default for Emu {
    fn default() -> Self {
        let mut ram = [0; RAM_SIZE]; //	Rust 允许省略 字段名: 变量名 形式，仅当二者相同 时（如 ram）。
        ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
        Self {
            pc: START_ADDR,
            ram,
            screen: [false; SCREEN_H * SCREEN_W],
            v_reg: [0; NUM_REGISTERS],
            i_reg: 0,
            sp: 0,
            stack: [0; STACK_SIZE],
            keys: [false; NUM_KEYS],
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

    fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }

    pub fn reset(&mut self) {
        let mut ram = [0; RAM_SIZE];
        ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);

        self.pc = START_ADDR;
        self.ram = ram;
        self.screen = [false; SCREEN_W * SCREEN_H];
        self.v_reg = [0; NUM_REGISTERS];
        self.i_reg = 0;
        self.sp = 0;
        self.stack = [0; STACK_SIZE];
        self.keys = [false; NUM_KEYS];
        self.dt = 0;
        self.st = 0;
        self.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
    }
    pub fn tick(&mut self) {
        let op = self.fetch();
        // Decode and Execute can happen simultaneously in the Chip-8 systems.
        self.execute(op);
    }
    fn execute(&mut self, op: u16) {
        let [digit1, digit2, digit3, digit4] = [
            (op >> 12) as u8,
            ((op >> 8) & 0xF) as u8,
            ((op >> 4) & 0xF) as u8,
            (op & 0xF) as u8,
        ];
        match (digit1, digit2, digit3, digit4) {
            // CLS
            (0, 0, 0xE, 0) => {
                self.screen = [false; SCREEN_H * SCREEN_W];
            }
            // RET
            (0, 0, 0xE, 0xE) => {
                let re_addr = self.pop();
                self.pc = re_addr;
            }
            (_, _, _, _) => unimplemented!("Unimplemented opcode: {}", op),
        }
    }
    pub fn tick_timers(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }
        if self.st > 0 {
            if self.st == 1 {
                //TODO
                // BEEP
                // Because this book does not implement this function, so it will finish later.
            }
        }
    }

    fn fetch(&mut self) -> u16 {
        // Use Big-Endian format for composing data.
        let higher_byte = self.ram[self.pc as usize] as u16;
        let lower_byte = self.ram[(self.pc + 1) as usize] as u16;
        let op = (higher_byte << 8) | lower_byte;
        self.pc += 2;
        op
    }
}
