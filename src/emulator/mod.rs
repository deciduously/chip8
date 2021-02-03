//! The Chip8 machine itself

mod context;
mod machine;
mod opcode;

#[cfg(feature = "sdl")]
pub use context::SdlContext;

#[cfg(feature = "wasm")]
pub use context::wasm;

pub use machine::Machine;
pub use opcode::*;
