//! The Chip8 machine itself

pub mod machine;
pub mod opcode;

// Constants

/// Starting memory location for the program to run - earlier cells are machine-reserved.
const PC_BEGIN: u16 = 0x200;
