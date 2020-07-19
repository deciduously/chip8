//! The Chip8 machine itself

pub mod machine;
pub mod opcode;

// Constants

/// Total memory available.
const MEM_SIZE: usize = 4096;
/// Number of registers avaialable for short-term storage.
const NUM_REGISTERS: usize = 16;
/// Keypad size.
const NUM_KEYS: usize = 16;
/// Screen height.
const PIXEL_ROWS: usize = 32;
/// Screen width.
const PIXEL_COLS: usize = 64;
/// Call stack depth.
const STACK_SIZE: usize = 16;
/// Helper const for the total number of screen pixels.
const TOTAL_PIXELS: usize = PIXEL_COLS * PIXEL_ROWS;
/// Starting memory location for the program to run - earlier cells are machine-reserved.
const PC_BEGIN: u16 = 0x200;
