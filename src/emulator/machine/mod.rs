//! The top-level software representation of the Chip8 virtual machine

use super::{opcode::*, *};
use anyhow::Result;
use std::{fs::File, io::Read, path::PathBuf};

#[cfg(test)]
mod test;

/// The sprites used to render hex digits:
/// ```txt
/// DEC   HEX    BIN         RESULT    DEC   HEX    BIN         RESULT
/// 240   0xF0   1111 0000    ****     240   0xF0   1111 0000    ****
/// 144   0x90   1001 0000    *  *      16   0x10   0001 0000       *
/// 144   0x90   1001 0000    *  *      32   0x20   0010 0000      *
/// 144   0x90   1001 0000    *  *      64   0x40   0100 0000     *
/// 240   0xF0   1111 0000    ****      64   0x40   0100 0000     *
/// ```
const FONTSET: [u8; 80] = [
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

/// The top-level software representation of the Chip8 machine
pub struct Machine {
    /// The current opcode
    opcode: Opcode,
    /// Available memory space - 4K
    /// 0x000-0x1FF - Chip 8 interpreter (contains font set in emu)
    /// 0x050-0x0A0 - Used for the built in 4x5 pixel font set (0-F)
    /// 0x200-0xFFF - Program ROM and work RAM
    // TODO: Should this be a Bytes?
    pub memory: [u8; MEM_SIZE],
    /// CPU Registers
    /// There are 15 general purpose registers, V0 through VE.
    /// The 16th register is the "carry" flag
    registers: [u8; NUM_REGISTERS],
    /// Index register
    pub idx: u16,
    /// Program counter
    pub pc: u16,
    /// Graphics system - 2048 total pixels, arranged 64x32
    screen: [u8; TOTAL_PIXELS],
    /// Delay timer - 60Hz, counts down if above 0
    delay_timer: u8,
    /// Sound timer - buzzes at 0.  60Hz, counts down if above 0\
    sound_timer: u8,
    /// Call stack
    stack: [usize; STACK_SIZE],
    /// Stack pointer
    sp: usize,
    /// Keep track of the keypad - 0x0-0xF
    key: [u8; NUM_KEYS],
}

impl Machine {
    // PUBLIC INTERFACE

    /// Initialize memory and registers.
    pub fn new() -> Self {
        // Stack, registers, memory, timers, and program counters all have sensible defaults
        let mut ret = Self::default();
        // Clear display
        // The fonts are the same for every game, we can just load once here.
        ret.load_fontset();
        ret
    }

    /// Retrieve the current byte.
    fn current_byte(&self) -> u8 {
        self.memory_get(self.pc)
    }

    /// Locate a program file by filename and load into memory.
    pub fn load_game(&mut self, name: &str) -> Result<usize> {
        // Clear the memory to make way
        // FIXME: This is useless if no game has previously been loaded, do I care?
        self.reset();

        // All the games live in the GAMES_DIR, have an uppercase name, and a .ch8 extension
        let game_path = Self::rom_path(name);

        // First, read the binary info.
        let mut file = File::open(&game_path)?;
        let mut buf: Vec<u8> = vec![];
        let bytes_read = file.read_to_end(&mut buf)?;

        // Load in memory starting at location 512 (0x200), which is where the pc pointer starts
        for (idx, &byte) in buf.iter().enumerate() {
            self.memory_set(idx as u16 + self.pc, byte);
        }

        println!("Loaded {} bytes from {:?}", bytes_read, game_path);
        Ok(bytes_read)
    }

    /// Run the machine.
    pub fn run(&mut self) -> Result<()> {
        loop {
            // Emulate one cycle
            self.cycle()?;
            // If the draw flag is set, update the screen
            // Store key press state
        }
        Ok(())
    }

    // PRIVATE/INTERNAL INTERFACE

    /// Set the carry flag to off
    pub fn carry_off(&mut self) {
        self.registers[0xF] = 0;
    }

    /// Set the carry flag to on
    pub fn carry_on(&mut self) {
        self.registers[0xF] = 1;
    }
    /// Get the current value of the carry flag
    pub fn carry_flag_set(&mut self) -> bool {
        self.registers[0xF] == 1
    }

    /// Emulate a single cycle of the Chip8 CPU.
    pub fn cycle(&mut self) -> Result<()> {
        // Grab the current opcode and copy it into this stack frame
        self.update_opcode()?;
        let code = self.opcode;
        // Execute it against the current machine
        code.execute(self);
        // Decrement timers if needed
        self.update_timers();
        Ok(())
    }

    /// Fetch the opcode specified by the program counter.
    fn fetch_opcode(&self) -> Result<Opcode> {
        // Consume two successive bytes, then combine for the opcode
        let first_byte = self.current_byte();
        let second_byte = self.memory_get(self.pc + 1);
        Ok(Opcode::new(first_byte, second_byte)?)
    }

    /// Initialization step to load the font bytes into main memory.
    fn load_fontset(&mut self) {
        for (idx, &byte) in FONTSET.iter().enumerate() {
            // Fonts go right at the beginning
            self.memory[idx] = byte;
        }
    }

    /// Get the byte at memory address x
    pub fn memory_get(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    /// Set the value at register x
    pub fn memory_set(&mut self, addr: u16, val: u8) {
        self.memory[addr as usize] = val;
    }

    /// Advance a single opcode
    pub fn next_opcode(&mut self) {
        self.pc += 2;
    }

    /// Push the current location onto the stack
    pub fn push_callsite(&mut self) {
        self.stack[self.sp] = self.pc as usize;
        self.sp += 1;
    }

    /// Get the value at register x
    pub fn register_get(&self, x: u8) -> u8 {
        self.registers[x as usize]
    }

    /// Set the value at register x
    pub fn register_set(&mut self, x: u8, val: u8) {
        self.registers[x as usize] = val;
    }

    /// Reset memory, registers, call stack for a new rom.
    fn reset(&mut self) {
        self.pc = PC_BEGIN;
        self.registers = [0; NUM_REGISTERS];
        self.memory = [0; MEM_SIZE];
        self.stack = [0; STACK_SIZE];
        self.delay_timer = 0;
        self.sound_timer = 0;
    }

    /// Update the opcode either with the passed value (for testing) or the current byte if None.
    /// Only public for testing, should not be used by other objects
    pub fn update_opcode(&mut self) -> Result<()> {
        self.opcode = self.fetch_opcode()?;
        Ok(())
    }

    /// Update the timer values
    fn update_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    // HELPERS

    /// Helper to pull the ROM relative filepath from the filename
    ///
    /// Example:
    /// ```
    /// # use chip8::emulator::machine::Machine;
    /// assert_eq!(Machine::rom_path("pong").to_str().unwrap(), "games/PONG.ch8")
    /// ```
    pub fn rom_path(name: &str) -> PathBuf {
        let mut game_path = PathBuf::from(GAMES_DIR);
        game_path.push(name.to_uppercase());
        game_path.set_extension(ROM_EXT);
        game_path
    }
}

impl Default for Machine {
    fn default() -> Self {
        Self {
            opcode: Opcode::default(),
            memory: [0; MEM_SIZE],
            registers: [0; NUM_REGISTERS],
            idx: 0,
            pc: PC_BEGIN,
            screen: [0; TOTAL_PIXELS],
            delay_timer: 0xFF,
            sound_timer: 0xFF,
            stack: [0; STACK_SIZE],
            sp: 0,
            key: [0; NUM_KEYS],
        }
    }
}
