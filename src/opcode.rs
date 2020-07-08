//! The opcodes suported by the Chip8 achitecture

use anyhow::anyhow;
use std::str::FromStr;

/// Each opcode is 2 bytes
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Opcode(u16);

impl Opcode {
    /// Use bitwise OR to combine two u8s into a u16
    /// http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/
    /// 0xA2       0xA2 << 8 = 0xA200   HEX
    /// 10100010   1010001000000000     BIN
    /// 1010001000000000 | // 0xA200
    /// 11110000 = // 0xF0 (0x00F0)
    /// ------------------
    /// 1010001011110000   // 0xA2F0
    pub fn from_adjacent(byte_one: u8, byte_two: u8) -> Self {
        Self((byte_one as u16) << 8 | byte_two as u16)
    }
}

impl FromStr for Opcode {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let decoded = hex::decode(s)?;
        if decoded.len() != 2 {
            Err(anyhow!("Opcodes are two bytes!  Received: {:?}", decoded))
        } else {
            Ok(Self::from_adjacent(decoded[0], decoded[1]))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use hex;
    use pretty_assertions::assert_eq;
    #[test]
    fn test_combine_opcodes() {
        let byte_one = hex::decode("A2").unwrap()[0];
        let byte_two = hex::decode("F0").unwrap()[0];
        assert_eq!(
            Opcode::from_adjacent(byte_one, byte_two),
            Opcode::from_str("A2F0").unwrap()
        )
    }
}
