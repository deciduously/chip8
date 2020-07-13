//! The Chip8 machine itself

pub mod machine;
pub mod opcode;

// Constants
const MEM_SIZE: usize = 4096;
const NUM_REGISTERS: usize = 16;
const NUM_KEYS: usize = 16;
const PIXEL_ROWS: usize = 32;
const PIXEL_COLS: usize = 64;
const STACK_SIZE: usize = 16;
const TOTAL_PIXELS: usize = PIXEL_COLS * PIXEL_ROWS;
