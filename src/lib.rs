//! A Chip8 VM as a library

mod emulator;
mod roms;

pub use emulator::{Machine, Opcode, RawOpcode};
pub use roms::ROMS;

#[cfg(feature = "sdl")]
pub use emulator::SdlContext;

#[cfg(feature = "wasm")]
pub use emulator::wasm::run;
