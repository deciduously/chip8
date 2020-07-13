//! The top-level software representation of the Chip8 virtual machine

use super::{opcode::*, *};
use anyhow::Result;

/// The top-level software representation of the Chip8 machine
pub struct Machine {
    /// The current opcode
    opcode: Opcode,
    /// Available memory space - 4K
    /// 0x000-0x1FF - Chip 8 interpreter (contains font set in emu)
    /// 0x050-0x0A0 - Used for the built in 4x5 pixel font set (0-F)
    /// 0x200-0xFFF - Program ROM and work RAM
    memory: [u8; MEM_SIZE],
    /// CPU Registers
    /// There are 15 general purpose registers, V0 through VE.
    /// The 16th register is the "carry" flag
    registers: [u8; NUM_REGISTERS],
    /// Index register
    idx: u16,
    /// Program counter
    pc: usize,
    /// Graphics system - 2048 total pixels, arranged 64x32
    screen: [u8; TOTAL_PIXELS],
    /// Delay timer - 60Hz, counts down if above 0
    delay_timer: u8,
    /// Sound timer - buzzes at 0.  60Hz, counts down if above 0\
    sound_timer: u8,
    /// Call stack
    stack: [u16; STACK_SIZE],
    /// Stack pointer
    sp: usize,
    /// Keep track of the keypad - 0x0-0xF
    key: [u8; NUM_KEYS],
}

impl Machine {
    /// Initialize memory and registers
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_game(&mut self, name: &str) {}

    pub fn run(&mut self) -> Result<()> {
        loop {
            // Emulate one cycle
            // If the draw flag is set, update the screen
            // Store key press state
        }
        Ok(())
    }

    /// PRIVATE

    /// Emulate a single cycle of the Chip8 CPU
    fn cycle(&mut self) {
        // Fetch Opcode
        // Decode Opcode
        // Execute Opcode
        // Update Timers
    }

    /// Fetch the opcode specified by the program counter
    fn fetch_opcode(&self) -> Result<Opcode> {
        // Consume two successive bytes, then combine for the opcode
        let first_byte = self.memory[self.pc];
        let second_byte = self.memory[self.pc + 1];
        Ok(Opcode::new(first_byte, second_byte)?)
    }
}

impl Default for Machine {
    fn default() -> Self {
        Self {
            opcode: Opcode::default(),
            memory: [0; MEM_SIZE],
            registers: [0; NUM_REGISTERS],
            idx: 0,
            pc: 0,
            screen: [0; TOTAL_PIXELS],
            delay_timer: 255,
            sound_timer: 255,
            stack: [0; STACK_SIZE],
            sp: 0,
            key: [0; NUM_KEYS],
        }
    }
}
