//! The opcodes suported by the Chip8 achitecture.
//!
//! Largely written by staring at [the Chip8 Wikipedia article](https://en.wikipedia.org/wiki/CHIP-8#Opcode_table) for a while.

/// All the supported types of opcode.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Opcode {
    /// 0NNN - Call machine code routine at addr NNN - usually unused in ROMs (in favor of 2NNN?).
    /// Carries NNN.
    MachineCall(u16),
    /// 00E0 - Clear the screen.
    Clear,
    /// 00EE - Return from a subroutine.
    Return,
    /// 1NNN - Jump to addr NNN.
    Jump(u16),
    /// 2NNN - Call subroutine at NNN.
    Call(u16),
    /// 3XNN - skip next instruction if VX == NN.  Carries (X, NN).
    SkipIfEqVal(u8, u8),
    /// 4XNN - Skip next if VX != NN.  Carries (X, NN).
    SkipIfNEqVal(u8, u8),
    /// 5XY0 - Skip next if VX == VY.  Carries (X, Y).
    SkipIfMatchReg(u8, u8),
    /// 6XNN - Set CV to NN.  Carries (X, NN).
    SetRegister(u8, u8),
    /// 7XNN - Add NN to VX, does not change carry flag.  Carries (X, NN).
    Add(u8, u8),
    /// 8XY0 - Set VX to VY.  Carries (X, Y).
    Assign(u8, u8),
    /// 8XY1 - Set VX to (VX | VY).  Carries (X, Y).
    AssignOr(u8, u8),
    /// 8XY2 - Set VX to (VX & VY).  Carries (X, Y).
    AssignAnd(u8, u8),
    /// 8XY3 - Set VX to (VX ^ VY).  Carries (X, Y).
    AssignXor(u8, u8),
    /// 8XY4 - VX += VY.  Set VF to 1 if there's a carry and 0 if there isn't.  Carries (X, Y).
    AddAssign(u8, u8),
    /// 8XY5 - VX -= VY.  Set VF to 0 if there's a borrow and 1 if there isn't.
    SubAssign(u8, u8),
    /// 8XY6 - Store least significant bit of VX to VF, shift VX right 1.  Carries X, Y unused.
    ShiftRight(u8),
    /// 8XY7 - Set VX=VY-VX.  Set VF to 0 if there's a borrow and one if there isn't.  Carries (X, Y).
    FlippedSubAssign(u8, u8),
    /// 8XYE - Store most significant bit of VX to VF, shift VX left 1.  Carries X, Y unusued.
    ShiftLeft(u8),
    /// 9XY0 - Skip next if VX != VY.  Carries (X, Y).
    SkipIfMismatchReg(u8, u8),
    /// ANNN - Set idx pointer to address NNN.  Carries NNN.
    SetI(u16),
    /// BNNN - Jump to address NNN plus V0.  Carries NNN.
    JumpTo(u16),
    /// CXNN - Pick a random number 0-255 as r, set VX to (r & NN).  Carries NN.
    Rand(u8),
    /// DXYN - Draw sprite at (VX, VY).  WIdth 8px, height Npx.
    /// Each row of 8px is read starting from location at the `idx` pointer, which doesn't change here.
    /// VF is set to 1 if any pixels are flipped from set to unset when the sprite is drawn, and 0 if not.
    /// Carries (X, Y, N).
    Draw(u8, u8, u8),
    /// EX9E - Skip next if key stored in VX is pressed.  Carries X.
    SkipIfPressed(u8),
    /// EXA1 - Skip next if key stored in VX is not pressed.  Carries X.
    SkipIfNotPressed(u8),
    /// FX07 - Set VX to the value of the delay timer.  Carries X.
    StoreDelay(u8),
    /// FX0A - Block until keypress, store key pressed to V0.
    WaitKey,
    /// FX15 - Set delay timer to VX.  Carries X.
    SetDelay(u8),
    /// FX18 - Set sound timer to VX.  Carries X.
    SetSound(u8),
    /// FX1E - Increment index pointer by VX.  Carries X.
    IncrementIdx(u8),
    /// FX29 - Set index pointer to the sprite address for the character in VX.
    ///Hex digis 0-F are all stores as 4x5 glyphs.
    /// Carries X.
    NewSprite(u8),
    /// FX33 - Store the binary-coded decimal representation of VX starting at the index pointer.
    /// ```rust
    /// let (hundreds_digit, tens_digit, ones_digit) = binary_coded_decimal(registers[x]);
    /// tape[idx+0] = hundreds_digit;
    /// tape[idx+1] = tens_digit;
    /// tape[idx+2] = ones_digit;
    /// ```
    /// Carries X.
    BCD(u8),
    /// FX55 - Store V0 to VX inclusive in memory starting at idx.  Leave idx itself unmodified.  Carries X.
    DumpRegisters(u8),
    /// FX65 - Fill V0 to VX inclusive in memory startinf from idx.  Leave idx itself unmodified.  Carries X.
    FillRegisters(u8),
    /// Unknown opcode, carries raw u16 passed
    Unrecognized(u16),
}

impl Opcode {
    /// Produce a single Opcode from two adjacent u8 values.
    /// e.g. 0x20 and 0xFF should combine to 0x20FF and return Opcode::Call()
    pub fn new(first: u8, second: u8) -> Self {
        Self::from(Self::combine_bytes(first, second))
    }
    /// Use bitwise OR to combine two u8s into a u16.
    /// Taken from [this post](http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/).
    /// ```
    /// 0xA2       0xA2 << 8 = 0xA200   HEX
    /// 10100010   1010001000000000     BIN
    /// 1010001000000000 | // 0xA200
    /// 11110000 = // 0xF0 (0x00F0)
    /// ------------------
    /// 1010001011110000   // 0xA2F0
    /// ```
    fn combine_bytes(byte_one: u8, byte_two: u8) -> u16 {
        (byte_one as u16) << 8 | byte_two as u16
    }
}

impl Default for Opcode {
    fn default() -> Self {
        Opcode::Unrecognized(0xFFFF)
    }
}

impl From<u16> for Opcode {
    fn from(raw: u16) -> Self {
        // This is the opcode lookup logic
        unimplemented!()
        // First, look at the first
        // let prefix = raw & 0xF000
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    #[test]
    fn test_combine_opcodes() {
        let byte_one = 0xA2;
        let byte_two = 0xF0;
        assert_eq!(Opcode::combine_bytes(byte_one, byte_two), 0xA2F0)
    }
    #[test]
    fn test_opcode_from_u16() {
        assert_eq!(Opcode::from(0x2F23), Opcode::Call(0xF23));
    }
}
