//! A Chip8 VM as a library

pub mod emulator;
mod roms;
pub use emulator::machine::*;

pub use roms::ROMS;

#[cfg(test)]
pub use emulator::machine::TestContext;
