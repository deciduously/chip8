//! The top-level software representation of the Chip8 virtual machine

use super::{super::ROMS, opcode::*, *};
use anyhow::{anyhow, Result};
use std::{
    fmt,
    sync::{Arc, RwLock},
};

#[cfg(test)]
use std::convert::TryFrom;

#[cfg(test)]
mod test;

use context::Context;

/// Total memory available.
const MEM_SIZE: usize = 4096;
/// Number of registers avaialable for short-term storage.
const NUM_REGISTERS: usize = 16;
/// Keypad size.
pub const NUM_KEYS: usize = 16;
/// Screen height.
pub const PIXEL_ROWS: u32 = 32;
/// Screen width.
pub const PIXEL_COLS: u32 = 64;
/// Call stack depth.
const STACK_SIZE: usize = 16;
/// Starting memory location for the program to run - earlier cells are machine-reserved.
const PC_BEGIN: u16 = 0x200;

/// Helper const for the total number of screen pixels.
const TOTAL_PIXELS: u32 = PIXEL_COLS * PIXEL_ROWS;

/// The pixel array
pub type Screen = [u8; TOTAL_PIXELS as usize];
pub const BLANK_SCREEN: Screen = [0; TOTAL_PIXELS as usize];

// TODO constant accessor for Screen??

/// The key state array.
/// This has to use thread-safe interior mutability to accommodate the Wasm event listener
#[derive(Debug, Clone)]
pub struct Keys {
    state: Arc<RwLock<[bool; NUM_KEYS]>>,
}

impl Default for Keys {
    fn default() -> Self {
        Self {
            state: Arc::new(RwLock::new([false; NUM_KEYS])),
        }
    }
}

impl Keys {
    pub fn new() -> Self {
        Self::default()
    }

    /// Depress a key
    pub fn key_down(&self, key: u8) {
        self.state.write().unwrap()[key as usize] = true;
    }

    /// Release a key
    pub fn key_up(&self, key: u8) {
        self.state.write().unwrap()[key as usize] = false;
    }

    /// Check if specific key is pressed
    pub fn is_pressed(&self, key: u8) -> bool {
        let key = key as usize;
        if key >= NUM_KEYS {
            false
        } else {
            self.state.read().unwrap()[key]
        }
    }

    // Get the internal state
    pub fn inner(&self) -> [bool; NUM_KEYS] {
        *self.state.read().unwrap()
    }
}

impl fmt::Display for Keys {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ret = String::new();
        for (idx, &key) in self.state.read().unwrap().iter().enumerate() {
            if key {
                ret.push_str(&format!("{:x} ", idx));
            }
        }
        if !ret.is_empty() {
            // Trim trailing space
            ret = ret[0..ret.len() - 1].to_string();
        }
        write!(f, "{}", ret)
    }
}

/// Helper to map a keyboard key to a hex key
///
///Keypad                   Keyboard
///
///|1|2|3|C| => |1|2|3|4|
///
///|4|5|6|D| => |Q|W|E|R|
///
///|7|8|9|E| => |A|S|D|F|
///
///|A|0|B|F| =>  |Z|X|C|V|
pub fn keyboard_to_keypad(keyboard: char) -> Result<u8> {
    match keyboard.to_ascii_uppercase() {
        '1' => Ok(1),
        '2' => Ok(2),
        '3' => Ok(3),
        '4' => Ok(0xC),
        'Q' => Ok(4),
        'W' => Ok(5),
        'E' => Ok(6),
        'R' => Ok(0xD),
        'A' => Ok(7),
        'S' => Ok(8),
        'D' => Ok(9),
        'F' => Ok(0xE),
        'Z' => Ok(0xA),
        'X' => Ok(0),
        'C' => Ok(0xB),
        'V' => Ok(0xF),
        _ => Err(anyhow!("Unsupported keyboard key")),
    }
}

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
    /// Trait objecct for interfacing with the outside world.
    context: Box<dyn Context>,
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
    pub idx: u16,
    /// Program counter
    pub pc: u16,
    /// Graphics system - 2048 total pixels, arranged 64x32
    screen: Screen,
    /// Flag to track whether we need to redraw
    pub draw_flag: bool,
    /// Delay timer - 60Hz, counts down if above 0
    pub delay_timer: u8,
    /// Sound timer - buzzes at 0.  60Hz, counts down if above 0\
    pub sound_timer: u8,
    /// Call stack, stores program counters of each call site
    pub stack: [u16; STACK_SIZE],
    /// Stack pointer
    pub sp: usize,
    /// Keep track of the keypad - 0x0-0xF
    key: Keys,
    /// The name of the currently loaded game
    pub current_game: Option<String>,
}

impl Machine {
    // PUBLIC INTERFACE

    /// Initialize memory and registers.
    pub fn new(context: Box<dyn Context>) -> Self {
        // Stack, registers, memory, timers, and program counters all have sensible defaults
        let mut ret = Self {
            context,
            opcode: Opcode::default(),
            memory: [0; MEM_SIZE],
            registers: [0; NUM_REGISTERS],
            idx: 0,
            pc: PC_BEGIN,
            screen: BLANK_SCREEN,
            draw_flag: true,
            delay_timer: 0xFF,
            sound_timer: 0xFF,
            stack: [0; STACK_SIZE],
            sp: 0,
            key: Keys::new(),
            current_game: None,
        };
        // The fonts are the same for every game, we can just load once here.
        ret.load_fontset();
        ret.context.init();
        ret
    }

    /// Locate a program file by filename and load into memory.
    pub fn load_game(&mut self, name: &str) -> Result<usize> {
        // Clear the memory to make way
        self.reset();

        // All the games live in the GAMES_DIR, have an uppercase name, and a .ch8 extension
        if let Some(rom) = ROMS.get(name) {
            self.current_game = Some(name.to_string());
            // Load in memory starting at location 512 (0x200), which is where the pc pointer starts
            for (idx, &byte) in rom.iter().enumerate() {
                self.memory_set(idx as u16 + self.pc, byte);
            }
            let num_bytes = rom.len();
            println!("Loaded {}: {} bytes", name, num_bytes);
            Ok(num_bytes)
        } else {
            Err(anyhow!("Game {} not included", name))
        }
    }

    /// Run the machine.
    pub fn run(&mut self) {
        loop {
            match self.step() {
                Ok(true) => break,
                Ok(_) => continue,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }

    /// Perform one step
    pub fn step(&mut self) -> Result<bool> {
        // Handle any events, quit if signaled
        if self.context.listen_for_input() {
            println!("Quitting...");
            return Ok(true);
        }
        // Only sleep if we're using native renderers.  WASM handles this on its own.
        #[cfg(not(feature = "wasm"))]
        self.context.sleep(2);

        //dbg!(self.keys_pressed_str());
        self.cycle()?;
        //println!("{:?}", self.opcode);
        // If the draw flag is set, update the screen
        if self.draw_flag {
            self.context.draw_graphics(self.screen);
            self.draw_flag = false;
        }
        // Store key press state
        self.set_keys(self.context.get_key_state());
        Ok(false)
    }

    // Pass through key_up and key_down
    pub fn key_down(&mut self, key: u8) {
        self.key.key_down(key);
    }

    // PRIVATE/INTERNAL INTERFACE

    /// Emit a beep
    fn beep(&self) {
        self.context.beep();
    }

    /// Set the carry flag to off
    fn carry_off(&mut self) {
        self.register_set(0xF, 0);
    }

    /// Set the carry flag to on
    fn carry_on(&mut self) {
        self.register_set(0xF, 1);
    }

    /// Check if the carry flag is set
    #[cfg(test)]
    fn carry_flag_set(&self) -> bool {
        self.register_get(0xF) == 1
    }

    /// Clear screen
    fn clear_screen(&mut self) {
        self.screen = BLANK_SCREEN;
    }

    /// Retrieve the current byte.
    fn current_byte(&self) -> u8 {
        self.memory_get(self.pc)
    }

    /// Emulate a single cycle of the Chip8 CPU.
    fn cycle(&mut self) -> Result<()> {
        // Grab the current opcode and copy it into this stack frame
        self.update_opcode()?;
        self.execute()?;
        // Decrement timers if needed
        self.update_timers();
        Ok(())
    }

    /// Execute the current opcode
    fn execute(&mut self) -> Result<()> {
        use Opcode::*;
        let code = self.opcode;
        match code {
            MachineCall(_addr) => todo!(),
            ClearScreen => {
                self.clear_screen();
                self.draw_flag = true;
                self.next_opcode();
            }
            Return => {
                self.pop_callsite();
                self.next_opcode();
            }
            Jump(addr) => self.pc = addr,
            Call(addr) => {
                // Store current location on the stack
                self.push_callsite();
                // Jump to new location
                self.pc = addr;
            }
            SkipIfEqVal(x, y) => {
                if self.register_get(x) == y {
                    // Extra advance
                    self.next_opcode();
                }
                // Always advance at least once
                self.next_opcode();
            }
            SkipIfNotEqVal(x, y) => {
                if self.register_get(x) != y {
                    // Extra advance
                    self.next_opcode();
                }
                // Always advance at least once
                self.next_opcode();
            }
            SkipIfMatchReg(x, y) => {
                if self.register_get(x) == self.register_get(y) {
                    // Extra advance
                    self.next_opcode();
                }
                // Always advance at least once
                self.next_opcode();
            }
            SetRegister(x, y) => {
                self.register_set(x, y);
                self.next_opcode();
            }
            Add(x, y) => {
                let current_x = self.register_get(x);
                //if current_x as u16 + y as u16 > u8::MAX as u16 {
                //    self.carry_on();
                //}
                self.register_set(x, current_x.wrapping_add(y));
                self.next_opcode();
            }
            Assign(x, y) => {
                self.register_set(x, self.register_get(y));
                self.next_opcode();
            }
            AssignOr(x, y) => {
                self.register_set(x, self.register_get(y) | self.register_get(x));
                self.next_opcode();
            }
            AssignAnd(x, y) => {
                self.register_set(x, self.register_get(y) & self.register_get(x));
                self.next_opcode();
            }
            AssignXor(x, y) => {
                self.register_set(x, self.register_get(y) ^ self.register_get(x));
                self.next_opcode();
            }
            AddAssign(x, y) => {
                let reg_x = self.register_get(x);
                let reg_y = self.register_get(y);

                // Check if the addition will overflow a byte, set carry flag and VX accordingly
                let add: u16 = reg_x as u16 + reg_y as u16;
                if add > 255 {
                    self.carry_on();
                } else {
                    self.carry_off();
                }
                self.register_set(x, (add & 0xFF) as u8);
                self.next_opcode();
            }
            SubAssign(x, y) => {
                let reg_x = self.register_get(x);
                let reg_y = self.register_get(y);

                // Check if the addition will drop below zero, set carry flag and VX accordingly
                if reg_y as i16 - reg_x as i16 > 0 {
                    self.carry_off(); // When it's a borrow, we set it to 0 subtract from the max byte
                } else {
                    self.carry_on();
                }
                self.register_set(x, reg_x.wrapping_sub(reg_y));
                self.next_opcode();
            }
            ShiftRight(x) => {
                let reg = self.register_get(x);
                // Set the carry flag according to LSB
                self.register_set(0xF, reg & 0x01);
                self.register_set(x, reg >> 1);
                self.next_opcode();
            }
            FlippedSubAssign(x, y) => {
                let reg_x = self.register_get(x);
                let reg_y = self.register_get(y);

                // Check if the addition will drop below zero, set carry flag and VX accordingly
                if reg_x as i16 > reg_y as i16 {
                    self.carry_off();
                } else {
                    self.carry_on();
                }
                self.register_set(x, reg_y.wrapping_sub(reg_x));
                self.next_opcode();
            }
            ShiftLeft(x) => {
                let reg = self.register_get(x);
                // Set the carry flag according to MSB
                // Shift by seven (number of bits in the byte minus 1), then it's the LSB!
                self.register_set(0xF, reg >> (8 - 1));
                self.register_set(x, reg << 1);
                self.next_opcode();
            }
            SkipIfMismatchReg(x, y) => {
                if self.register_get(x) != self.register_get(y) {
                    self.next_opcode();
                }
                self.next_opcode();
            }
            SetIdx(addr) => {
                self.idx = addr;
                self.next_opcode();
            }
            JumpTo(addr) => self.pc = addr + self.register_get(0) as u16,
            Rand(x, mask) => {
                let r = self.context.random_byte();
                self.register_set(x, r & mask);
                self.next_opcode();
            }
            Draw(x, y, h) => {
                let reg_x = self.register_get(x);
                let reg_y = self.register_get(y);
                // reset collision detection register - uses carry flag
                self.carry_off();
                // Loop over each row
                for yline in 0..h {
                    let row = ((reg_y as u32 + yline as u32) % PIXEL_ROWS) as u8;
                    // Fetch pixel value
                    let pixel = self.memory_get(self.idx + (yline as u16));
                    // Loop over each bit in the row
                    for xline in 0..8 {
                        let col = ((reg_x as u32 + xline as u32) % PIXEL_COLS) as u8;
                        // Check if current pixel is set to one, i.e. we need to draw it.
                        // 0x80 >> xline scans through to the current bit
                        if pixel & (0x80 >> xline) != 0 {
                            // Check if the current display pixel is also set to 1
                            if self.screen_get(col, row) == 1 {
                                // Collision detected!
                                self.carry_on();
                            }
                            // Set the pixel value with XOR
                            self.screen_set(col, row);
                        }
                    }
                }
                // We updated the screen, trigger redraw
                self.draw_flag = true;
                self.next_opcode();
            }
            SkipIfPressed(key_reg) => {
                let key = self.register_get(key_reg);
                if self.key_pressed(key) {
                    self.next_opcode();
                }
                self.next_opcode();
            }
            SkipIfNotPressed(key_reg) => {
                let key = self.register_get(key_reg);
                if !self.key_pressed(key) {
                    self.next_opcode();
                }
                self.next_opcode();
            }
            StoreDelay(x) => {
                self.register_set(x, self.delay_timer);
                self.next_opcode();
            }
            WaitKey(x) => {
                // Wait for any key to be pressed
                let mut key_press = None;

                // check through keys
                for (idx, pressed) in self.key.state.write().unwrap().iter_mut().enumerate() {
                    if *pressed {
                        key_press = Some(idx as u8);
                        break;
                    }
                }

                if let Some(key) = key_press {
                    self.register_set(x, key);
                    self.next_opcode();
                } else {
                    // If we didn't find it, skip this cycle and try again
                    return Ok(());
                }
            }
            SetDelay(x) => {
                self.delay_timer = self.register_get(x);
                self.next_opcode();
            }
            SetSound(x) => {
                self.sound_timer = self.register_get(x);
                self.next_opcode();
            }
            IncrementIdx(x) => {
                let curr = self.register_get(x) as u16;
                if self.idx + curr > 0xFFF {
                    self.carry_on();
                } else {
                    self.carry_off();
                }
                self.idx += curr;
                self.idx &= 0xFFF;
                self.next_opcode();
            }
            NewSprite(x) => {
                self.idx = (self.register_get(x) * 0x5) as u16;
                self.next_opcode();
            }
            BCD(x) => {
                let reg_x = self.register_get(x);
                self.memory_set(self.idx, reg_x / 100);
                self.memory_set(self.idx + 1, (reg_x / 10) % 10);
                self.memory_set(self.idx + 2, (reg_x % 100) % 10);
                self.next_opcode();
            }
            DumpRegisters(x) => {
                let start_idx = self.idx;
                for i in 0..=x {
                    self.memory_set(start_idx + i as u16, self.register_get(i));
                }
                // Superchip leaves this unmodifed
                //self.idx += (x + 1) as u16;
                self.next_opcode();
            }
            FillRegisters(x) => {
                let start_idx = self.idx;
                for i in 0..=x {
                    self.register_set(i, self.memory_get(start_idx + i as u16));
                }
                // Superchip leaves this unmodified
                //self.idx += (x + 1) as u16;
                self.next_opcode();
            }
        }
        Ok(())
    }

    /// Fetch the opcode specified by the program counter.
    fn fetch_opcode(&self) -> Result<Opcode> {
        // Consume two successive bytes, then combine for the opcode
        let first_byte = self.current_byte();
        let second_byte = self.memory_get(self.pc + 1);
        Ok(Opcode::new(first_byte, second_byte)?)
    }

    /// Check if given key is pressed
    fn key_pressed(&self, key: u8) -> bool {
        self.key.is_pressed(key)
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

    /// Pop the top value off the call stack
    fn pop_callsite(&mut self) {
        // Reduce the pointer
        self.sp -= 1;
        // Grab previous addressto return
        let ret = self.stack[self.sp];
        // Clear the stack slot
        self.stack[self.sp] = 0;
        self.pc = ret;
    }

    /// Push the current location onto the stack
    fn push_callsite(&mut self) {
        self.stack[self.sp] = self.pc;
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
        self.draw_flag = true;
        self.screen = BLANK_SCREEN;
        self.load_fontset();
    }

    /// Get the value at screen position (x, y)
    fn screen_get(&self, x: u8, y: u8) -> u8 {
        self.screen[x as usize + (y as u32 * PIXEL_COLS) as usize]
    }

    /// Draw the pixel at screen position (x, y)
    fn screen_set(&mut self, x: u8, y: u8) {
        self.screen[x as usize + ((y as u32 * PIXEL_COLS) as usize)] ^= 1;
    }

    /// Store a newly read key state
    fn set_keys(&mut self, keys: [bool; NUM_KEYS]) {
        *self.key.state.write().unwrap() = keys;
    }

    /// Update the opcode either with the passed value (for testing) or the current byte if None.
    fn update_opcode(&mut self) -> Result<()> {
        self.opcode = self.fetch_opcode()?;
        Ok(())
    }

    /// Update the timer values
    fn update_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                self.beep();
            }
            self.sound_timer -= 1;
        }
    }

    /// Set an opcode and immediate execute it, for testing purposes
    #[cfg(test)]
    pub fn test_opcode(&mut self, opcode: u16) {
        self.opcode = Opcode::try_from(opcode).unwrap();
        self.execute().unwrap();
    }
}
