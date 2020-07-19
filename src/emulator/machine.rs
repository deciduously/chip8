//! The top-level software representation of the Chip8 virtual machine

use super::{opcode::*, *};
use anyhow::Result;
use lazy_static::lazy_static;
use std::{fs::File, io::Read, path::PathBuf};

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

lazy_static! {
    /// The location on disk for the available ROMS to load is currently just hardcoded
    pub static ref GAMES_DIR: PathBuf = PathBuf::from("games");
}

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
    idx: u16,
    /// Program counter
    pc: u16,
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

        // All the games live in the GAMES_DIR
        let mut game_path = GAMES_DIR.clone();
        game_path.push(name.to_uppercase());

        // First, read the binary info.
        let mut file = File::open(game_path)?;
        let mut buf: Vec<u8> = vec![];
        let bytes_read = file.read_to_end(&mut buf)?;

        // Load in memory starting at location 512 (0x200), which is where the pc pointer starts
        for (idx, &byte) in buf.iter().enumerate() {
            self.memory_set(idx as u16 + self.pc, byte);
        }

        println!("Loaded {} bytes from {}", bytes_read, name);
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

    // PRIVATE

    /// Set the carry flag to off
    fn carry_off(&mut self) {
        self.registers[0xF] = 0;
    }

    /// Set the carry flag to on
    fn carry_on(&mut self) {
        self.registers[0xF] = 1;
    }
    /// Get the current value of the carry flag
    fn carry_flag_set(&mut self) -> bool {
        self.registers[0xF] == 1
    }

    /// Emulate a single cycle of the Chip8 CPU.
    fn cycle(&mut self) -> Result<()> {
        // passing `None` means it should read a new opcode using the built-in program counter
        self.update_opcode(None)?;
        self.execute_opcode();
        self.update_timers();
        Ok(())
    }

    /// Perform an opcode.
    fn execute_opcode(&mut self) {
        use Opcode::*;
        match self.opcode {
            MachineCall(addr) => {}
            ClearScreen => {}
            Return => {}
            Jump(addr) => {}
            Call(addr) => self.call(addr),
            SkipIfEqVal(x, y) => {}
            SkipIfNotEqVal(x, y) => {}
            SkipIfMatchReg(x, y) => {}
            SetRegister(x, y) => {}
            Add(x, y) => {}
            Assign(x, y) => {}
            AssignOr(x, y) => {}
            AssignAnd(x, y) => {}
            AssignXor(x, y) => {}
            AddAssign(x, y) => self.add_assign(x, y),
            SubAssign(x, y) => {}
            ShiftRight(x) => {}
            FlippedSubAssign(x, y) => {}
            ShiftLeft(x) => {}
            SkipIfMismatchReg(x, y) => {}
            SetIdx(addr) => self.set_idx(addr),
            JumpTo(addr) => {}
            Rand(x, mask) => {}
            Draw(x, y, h) => {}
            SkipIfPressed(key) => {}
            SkipIfNotPressed(key) => {}
            StoreDelay(x) => {}
            WaitKey => {}
            SetDelay(x) => {}
            SetSound(x) => {}
            IncrementIdx(x) => {}
            NewSprite(x) => {}
            BCD(x) => self.binary_coded_decimal(x),
            DumpRegisters(x) => {}
            FillRegisters(x) => {}
        }
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
    fn memory_get(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    /// Set the value at register x
    fn memory_set(&mut self, addr: u16, val: u8) {
        self.memory[addr as usize] = val;
    }

    /// Advance a single opcode
    fn next_opcode(&mut self) {
        self.pc += 2;
    }

    /// Push the current location onto the stack
    fn push_callsite(&mut self) {
        self.stack[self.sp] = self.pc as usize;
        self.sp += 1;
    }

    /// Get the value at register x
    fn register_get(&self, x: u8) -> u8 {
        self.registers[x as usize]
    }

    /// Set the value at register x
    fn register_set(&mut self, x: u8, val: u8) {
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
    pub fn update_opcode(&mut self, opcode: Option<Opcode>) -> Result<()> {
        let new_op = match opcode {
            Some(code) => code,
            None => self.fetch_opcode()?,
        };
        self.opcode = new_op;
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

    // OPCODE FNS

    /// Call subroutine at addr
    fn call(&mut self, addr: u16) {
        // Store current location on the stack
        self.push_callsite();
        // Jump to new location
        self.pc = addr;
    }

    /// Set index pointer to addr.
    fn set_idx(&mut self, addr: u16) {
        // Store index
        self.idx = addr;
        self.next_opcode();
    }

    /// Add the contents of VY to VX, setting the carry flag if it wraps over a u8
    fn add_assign(&mut self, x: u8, y: u8) {
        let reg_x = self.register_get(x);
        let reg_y = self.register_get(y);

        // Check if the addition will overflow a byte, set carry flag and VX accordingly
        let headroom = 0xFF - reg_x;
        if reg_y > headroom {
            self.carry_on();
            self.register_set(x, reg_y - headroom);
        } else {
            self.carry_off();
            self.register_set(x, reg_x + reg_y);
        }
        self.next_opcode();
    }

    /// Store the binary coded decimal representation of the value at VX to memory at idx, idx + 1, and idx + 2.
    ///
    /// Shamelessly stolen from the Opcode Examples section of [this post](http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/), Example 3.
    fn binary_coded_decimal(&mut self, x: u8) {
        let reg_x = self.register_get(x);
        self.memory_set(self.idx, reg_x / 100);
        self.memory_set(self.idx + 1, (reg_x / 10) % 10);
        self.memory_set(self.idx + 2, (reg_x % 100) % 10);
        self.next_opcode();
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

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::convert::TryFrom;
    #[test]
    fn test_load_fonts() {
        let machine = Machine::new();
        // The constructor should properly load the full fontset
        assert_eq!(machine.memory_get(0), 0xF0);
        assert_eq!(machine.memory_get(79), 0x80);
    }
    #[test]
    fn test_load_game() {
        let mut machine = Machine::new();
        let bytes = machine.load_game("pong").unwrap();
        assert_eq!(bytes, 246);
        assert_eq!(machine.current_byte(), 0x6A)
    }
    #[test]
    fn test_load_second_game() {
        // Should clear memory and load the new game
        let mut machine = Machine::new();
        let _: usize = machine.load_game("pong").unwrap();
        let bytes = machine.load_game("tank").unwrap();
        assert_eq!(bytes, 560);
        assert_eq!(machine.current_byte(), 0x12)
    }
    #[test]
    fn test_game_not_found() {
        let mut machine = Machine::new();
        assert_eq!(
            machine.load_game("ping").err().unwrap().to_string(),
            "No such file or directory (os error 2)".to_string()
        );
    }
    #[test]
    fn test_opcode_2nnn_call() {
        let mut machine = Machine::new();
        machine
            .update_opcode(Some(Opcode::try_from(0x2BCD).unwrap()))
            .unwrap();
        machine.execute_opcode();
        // Should store the current location in the stack to jump back later
        assert_eq!(machine.stack[0], PC_BEGIN as usize);
        // Should increment stack pointer
        assert_eq!(machine.sp, 1);
        // Should set program counter to new location
        assert_eq!(machine.pc, 0xBCD);
    }
    #[test]
    fn test_opcode_annn_set_idx() {
        let mut machine = Machine::new();
        machine
            .update_opcode(Some(Opcode::try_from(0xABCD).unwrap()))
            .unwrap();
        machine.execute_opcode();
        // Should store index given
        assert_eq!(machine.idx, 0xBCD);
        // Should increment program counter by two
        assert_eq!(machine.pc, PC_BEGIN + 2);
    }
    #[test]
    fn test_opcode_8xy4_add_assign() {
        let mut machine = Machine::new();
        // Seed registers
        machine.registers[0xB] = 3;
        machine.registers[0xC] = 15;
        machine
            .update_opcode(Some(Opcode::try_from(0x8BC4).unwrap()))
            .unwrap();
        machine.execute_opcode();

        // Should add VY to VX, wrapping around 0xFF
        assert_eq!(machine.register_get(0xB), 18);
        // Should not affect VY
        assert_eq!(machine.register_get(0xC), 15);
        // Should not set carry flag
        assert!(!machine.carry_flag_set());
        // Should increment program counter by two
        assert_eq!(machine.pc, PC_BEGIN + 2);
    }
    #[test]
    fn test_opcode_8xy4_add_assign_with_carry() {
        let mut machine = Machine::new();
        // Seed registers - each is only one byte, so this will wrap over
        machine.registers[0xB] = 250;
        machine.registers[0xC] = 15;
        machine
            .update_opcode(Some(Opcode::try_from(0x8BC4).unwrap()))
            .unwrap();
        machine.execute_opcode();

        // Should add VY to VX, wrapping around 0xFF
        assert_eq!(machine.register_get(0xB), 10);
        // Should not affect VY
        assert_eq!(machine.register_get(0xC), 15);
        // Should set carry flag
        assert!(machine.carry_flag_set());
        // Should increment program counter by two
        assert_eq!(machine.pc, PC_BEGIN + 2);
    }
    #[test]
    fn test_opcode_fx33_bcd() {
        let mut machine = Machine::new();
        machine.registers[0xB] = 195;
        machine.idx = 0xAB;
        machine
            .update_opcode(Some(Opcode::try_from(0xFB33).unwrap()))
            .unwrap();
        machine.execute_opcode();
        assert_eq!(machine.memory[0xAB], 1);
        assert_eq!(machine.memory[0xAB + 1], 9);
        assert_eq!(machine.memory[0xAB + 2], 5);
    }
}
