//! The opcodes suported by the Chip8 achitecture.
//!
//! Largely written by staring at [the Chip8 Wikipedia article](https://en.wikipedia.org/wiki/CHIP-8#Opcode_table) for a while.

use super::machine::Machine;
use anyhow::{anyhow, Result};
use std::{convert::TryFrom, fmt};

/// Wrapper struct with some helper methods for working with u16 values
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RawOpcode(u16);

impl RawOpcode {
    /// Construct a new opcode from two bytes.
    /// ```
    /// # use chip8::emulator::opcode::RawOpcode;
    /// # use pretty_assertions::assert_eq;
    /// assert_eq!(u16::from(RawOpcode::new(0xAB, 0xCD)), 0xABCD);
    /// ```
    pub fn new(first: u8, second: u8) -> Self {
        Self::from(Self::combine_bytes(first, second))
    }

    /// Use bitwise OR to combine two u8s into a u16.
    /// ```
    /// # use chip8::emulator::opcode::RawOpcode;
    /// # use pretty_assertions::assert_eq;
    /// assert_eq!(RawOpcode::combine_bytes(0xA2, 0xF0), 0xA2F0);
    /// ```
    pub fn combine_bytes(byte_one: u8, byte_two: u8) -> u16 {
        (byte_one as u16) << 8 | byte_two as u16
    }

    /// Get a hex digit from a 16 bit value from the most significant.
    /// ```
    /// # use chip8::emulator::opcode::RawOpcode;
    /// # use pretty_assertions::assert_eq;
    /// let code = RawOpcode::from(0xABCD);
    /// assert_eq!(code.hex_digit_from_left(0), 0xA);
    /// assert_eq!(code.hex_digit_from_left(1), 0xB);
    /// assert_eq!(code.hex_digit_from_left(2), 0xC);
    /// assert_eq!(code.hex_digit_from_left(3), 0xD);
    /// ```
    /// Panics if passed anything higher than 3 "cannot get the 4th hex digit from 4-digit number 0xABCD"
    pub fn hex_digit_from_left(&self, from_most: u8) -> u8 {
        // Shift over by proper amount of bytes, and then zero out the bits to the left with &
        // This will make any bit that isn't flipped a 0.
        // The result will fit in a u8
        if from_most > 3 {
            panic!(
                "cannot get the {}th hex digit from 4-digit number {:#0x}",
                from_most, self.0
            );
        };
        let bits = 4 * (3 - from_most);
        ((self.0 >> bits) & 0xF) as u8
    }

    /// Get the second and third digits.
    /// ```
    /// # use chip8::emulator::opcode::RawOpcode;
    /// # use pretty_assertions::assert_eq;
    /// assert_eq!(RawOpcode::from(0xABCD).middle_digits(), (0xB, 0xC));
    /// ```
    pub fn middle_digits(&self) -> (u8, u8) {
        (self.hex_digit_from_left(1), self.hex_digit_from_left(2))
    }

    /// Get the least significant byte form a 16 bit value.
    /// ```
    /// # use chip8::emulator::opcode::RawOpcode;
    /// # use pretty_assertions::assert_eq;
    /// assert_eq!(RawOpcode::from(0xABCD).last_byte(), 0xCD);
    /// ```
    pub fn last_byte(&self) -> u8 {
        (self.0 & 0x00FF) as u8
    }

    /// Get all but the first digit.
    /// ```
    /// # use chip8::emulator::opcode::RawOpcode;
    /// # use pretty_assertions::assert_eq;
    /// assert_eq!(RawOpcode::from(0xABCD).last_three_digits(), 0xBCD);
    /// ```
    pub fn last_three_digits(&self) -> u16 {
        self.0 & 0x0FFF
    }
}

impl From<u16> for RawOpcode {
    fn from(x: u16) -> Self {
        Self(x)
    }
}

impl From<RawOpcode> for u16 {
    fn from(raw: RawOpcode) -> Self {
        raw.0
    }
}

impl fmt::Display for RawOpcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// All the supported types of opcode.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Opcode {
    /// 0NNN - Call machine code routine at addr NNN - usually unused in ROMs (in favor of 2NNN?).
    /// Carries NNN.
    MachineCall(u16),
    /// 00E0 - Clear the screen.
    ClearScreen,
    /// 00EE - Return from a subroutine.
    Return,
    /// 1NNN - Jump to addr NNN.
    Jump(u16),
    /// 2NNN - Call subroutine at NNN.
    Call(u16),
    /// 3XNN - skip next instruction if VX == NN.  Carries (X, NN).
    SkipIfEqVal(u8, u8),
    /// 4XNN - Skip next if VX != NN.  Carries (X, NN).
    SkipIfNotEqVal(u8, u8),
    /// 5XY0 - Skip next if VX == VY.  Carries (X, Y).
    SkipIfMatchReg(u8, u8),
    /// 6XNN - Set VX to NN.  Carries (X, NN).
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
    /// 8XY5 - VX -= VY.  Set VF to 0 if there's a borrow and 1 if there isn't.  Carries (X, Y).
    SubAssign(u8, u8),
    /// 8XY6 - Store least significant bit of VX to VF, shift VX right 1.  Carries X, Y is unused.
    ShiftRight(u8),
    /// 8XY7 - Set VX=VY-VX.  Set VF to 0 if there's a borrow and one if there isn't.  Carries (X, Y).
    FlippedSubAssign(u8, u8),
    /// 8XYE - Store most significant bit of VX to VF, shift VX left 1.  Carries X, Y is unusued.
    ShiftLeft(u8),
    /// 9XY0 - Skip next if VX != VY.  Carries (X, Y).
    SkipIfMismatchReg(u8, u8),
    /// ANNN - Set idx pointer to address NNN.  Carries NNN.
    SetIdx(u16),
    /// BNNN - Jump to address NNN plus V0.  Carries NNN.
    JumpTo(u16),
    /// CXNN - Pick a random number 0-255 as r, set VX to (r & NN).  Carries (X, NN).
    Rand(u8, u8),
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
    /// Hex digis 0-F are all stores as 4x5 glyphs.
    /// Carries X.
    NewSprite(u8),
    /// FX33 - Store the binary-coded decimal representation of VX starting at the index pointer.
    /// Hundreds digit at `tape[idx]`, tens to `tape[idx+1]`, ones to `tape[idx+2]`.
    /// Carries X.
    BCD(u8),
    /// FX55 - Store V0 to VX inclusive in memory starting at idx.  Leave idx itself unmodified.  Carries X.
    DumpRegisters(u8),
    /// FX65 - Fill V0 to VX inclusive in memory starting from idx.  Leave idx itself unmodified.  Carries X.
    FillRegisters(u8),
}

impl Opcode {
    /// Produce a single Opcode from two adjacent u8 (byte) values.
    /// ```
    /// # use chip8::emulator::opcode::*;
    /// # use pretty_assertions::assert_eq;
    /// assert_eq!(Opcode::new(0x0A, 0xBC).unwrap(), Opcode::MachineCall(0xABC));
    /// assert_eq!(Opcode::new(0x00, 0xE0).unwrap(), Opcode::ClearScreen);
    /// assert_eq!(Opcode::new(0x00, 0xEE).unwrap(), Opcode::Return);
    /// assert_eq!(Opcode::new(0x1F, 0xFF).unwrap(), Opcode::Jump(0xFFF));
    /// assert_eq!(Opcode::new(0x2F, 0xFF).unwrap(), Opcode::Call(0xFFF));
    /// assert_eq!(Opcode::new(0x32, 0xFF).unwrap(), Opcode::SkipIfEqVal(2, 0xFF));
    /// assert_eq!(Opcode::new(0x42, 0xFF).unwrap(), Opcode::SkipIfNotEqVal(2, 0xFF));
    /// assert_eq!(Opcode::new(0x52, 0x30).unwrap(), Opcode::SkipIfMatchReg(2, 3));
    /// assert_eq!(Opcode::new(0x62, 0xBC).unwrap(), Opcode::SetRegister(2, 0xBC));
    /// assert_eq!(Opcode::new(0x79, 0xED).unwrap(), Opcode::Add(9,0xED));
    /// assert_eq!(Opcode::new(0x8B, 0xC0).unwrap(), Opcode::Assign(0xB,0xC));
    /// assert_eq!(Opcode::new(0x8B, 0xC1).unwrap(), Opcode::AssignOr(0xB,0xC));
    /// assert_eq!(Opcode::new(0x8B, 0xC2).unwrap(), Opcode::AssignAnd(0xB,0xC));
    /// assert_eq!(Opcode::new(0x8B, 0xC3).unwrap(), Opcode::AssignXor(0xB,0xC));
    /// assert_eq!(Opcode::new(0x8B, 0xC4).unwrap(), Opcode::AddAssign(0xB,0xC));
    /// assert_eq!(Opcode::new(0x8B, 0xC5).unwrap(), Opcode::SubAssign(0xB,0xC));
    /// assert_eq!(Opcode::new(0x8B, 0xC6).unwrap(), Opcode::ShiftRight(0xB));
    /// assert_eq!(Opcode::new(0x8B, 0xC7).unwrap(), Opcode::FlippedSubAssign(0xB,0xC));
    /// assert_eq!(Opcode::new(0x8B, 0xCE).unwrap(), Opcode::ShiftLeft(0xB));
    /// assert_eq!(Opcode::new(0x9B, 0xC0).unwrap(), Opcode::SkipIfMismatchReg(0xB, 0xC));
    /// assert_eq!(Opcode::new(0xAF, 0xAB).unwrap(), Opcode::SetIdx(0xFAB));
    /// assert_eq!(Opcode::new(0xBF, 0xAB).unwrap(), Opcode::JumpTo(0xFAB));
    /// assert_eq!(Opcode::new(0xCF, 0xAB).unwrap(), Opcode::Rand(0xF, 0xAB));
    /// assert_eq!(Opcode::new(0xD9, 0xAF).unwrap(), Opcode::Draw(9, 0xA, 0xF));
    /// assert_eq!(Opcode::new(0xEB, 0x9E).unwrap(), Opcode::SkipIfPressed(0xB));
    /// assert_eq!(Opcode::new(0xEB, 0xA1).unwrap(), Opcode::SkipIfNotPressed(0xB));
    /// assert_eq!(Opcode::new(0xFB, 0x07).unwrap(), Opcode::StoreDelay(0xB));
    /// assert_eq!(Opcode::new(0xFB, 0x0A).unwrap(), Opcode::WaitKey);
    /// assert_eq!(Opcode::new(0xFB, 0x15).unwrap(), Opcode::SetDelay(0xB));
    /// assert_eq!(Opcode::new(0xFB, 0x18).unwrap(), Opcode::SetSound(0xB));
    /// assert_eq!(Opcode::new(0xFB, 0x1E).unwrap(), Opcode::IncrementIdx(0xB));
    /// assert_eq!(Opcode::new(0xFB, 0x29).unwrap(), Opcode::NewSprite(0xB));
    /// assert_eq!(Opcode::new(0xFF, 0x33).unwrap(), Opcode::BCD(0xF));
    /// assert_eq!(Opcode::new(0xFF, 0x55).unwrap(), Opcode::DumpRegisters(0xF));
    /// assert_eq!(Opcode::new(0xFF, 0x65).unwrap(), Opcode::FillRegisters(0xF));
    /// ```
    /// Will pass up the raw opcode in an error if it doesn't match the table.
    /// ```should_panic
    /// # use chip8::emulator::opcode::*;
    /// # use pretty_assertions::assert_eq;
    /// assert_eq!(Opcode::new(0x8B, 0xCF).err().unwrap().to_string(), "Invalid Code: 0x8BCF".to_string());
    /// ```
    pub fn new(first: u8, second: u8) -> Result<Self> {
        Ok(Self::try_from(RawOpcode::new(first, second))?)
    }

    /// Apply this opcode to a machine instance
    pub fn execute(&self, machine: &mut Machine) {
        use Opcode::*;
        match *self {
            MachineCall(addr) => {}
            ClearScreen => {}
            Return => {}
            Jump(addr) => {}
            Call(addr) => {
                // Store current location on the stack
                machine.push_callsite();
                // Jump to new location
                machine.pc = addr;
            }
            SkipIfEqVal(x, y) => {}
            SkipIfNotEqVal(x, y) => {}
            SkipIfMatchReg(x, y) => {}
            SetRegister(x, y) => {
                machine.register_set(x, y);
                machine.next_opcode();
            }
            Add(x, y) => {
                machine.register_set(x, machine.register_get(x) + y);
                machine.next_opcode();
            }
            Assign(x, y) => {
                machine.register_set(x, machine.register_get(y));
                machine.next_opcode();
            }
            AssignOr(x, y) => {
                machine.register_set(x, machine.register_get(y) | machine.register_get(x));
                machine.next_opcode();
            }
            AssignAnd(x, y) => {
                machine.register_set(x, machine.register_get(y) & machine.register_get(x));
                machine.next_opcode();
            }
            AssignXor(x, y) => {
                machine.register_set(x, machine.register_get(y) ^ machine.register_get(x));
                machine.next_opcode();
            }
            AddAssign(x, y) => {
                let reg_x = machine.register_get(x);
                let reg_y = machine.register_get(y);

                // Check if the addition will overflow a byte, set carry flag and VX accordingly
                let headroom = 0xFF - reg_x;
                if reg_y > headroom {
                    machine.carry_on();
                    machine.register_set(x, reg_y - headroom);
                } else {
                    machine.carry_off();
                    machine.register_set(x, reg_x + reg_y);
                }
                machine.next_opcode();
            }
            SubAssign(x, y) => {
                let reg_x = machine.register_get(x);
                let reg_y = machine.register_get(y);

                // Check if the addition will drop below zero, set carry flag and VX accordingly
                if reg_y as i16 - reg_x as i16 > 0 {
                    machine.carry_off(); // When it's a borrow, we set it to 0 subtract from the max byte
                    machine.register_set(x, 0xFF - (reg_y - reg_x - 1));
                } else {
                    machine.carry_on();
                    machine.register_set(x, reg_x - reg_y);
                }
                machine.next_opcode();
            }
            ShiftRight(x) => {}
            FlippedSubAssign(x, y) => {}
            ShiftLeft(x) => {}
            SkipIfMismatchReg(x, y) => {}
            SetIdx(addr) => {
                machine.idx = addr;
                machine.next_opcode();
            }
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
            BCD(x) => {
                let reg_x = machine.register_get(x);
                machine.memory_set(machine.idx, reg_x / 100);
                machine.memory_set(machine.idx + 1, (reg_x / 10) % 10);
                machine.memory_set(machine.idx + 2, (reg_x % 100) % 10);
                machine.next_opcode();
            }
            DumpRegisters(x) => {}
            FillRegisters(x) => {}
        }
    }
}

impl Default for Opcode {
    fn default() -> Self {
        Opcode::WaitKey
    }
}

impl TryFrom<RawOpcode> for Opcode {
    type Error = anyhow::Error;
    fn try_from(raw: RawOpcode) -> Result<Self, Self::Error> {
        use Opcode::*;
        let error_val = Err(anyhow!("Invalid opcode {}", raw));
        match raw.hex_digit_from_left(0) {
            0 => {
                let addr = raw.last_three_digits();
                match addr {
                    0x0E0 => Ok(ClearScreen),
                    0x0EE => Ok(Return),
                    _ => Ok(MachineCall(addr)),
                }
            }
            1 => Ok(Jump(raw.last_three_digits())),
            2 => Ok(Call(raw.last_three_digits())),
            3 => Ok(SkipIfEqVal(raw.hex_digit_from_left(1), raw.last_byte())),
            4 => Ok(SkipIfNotEqVal(raw.hex_digit_from_left(1), raw.last_byte())),
            5 => {
                if raw.hex_digit_from_left(3) != 0 {
                    error_val
                } else {
                    let (x, y) = raw.middle_digits();
                    Ok(SkipIfMatchReg(x, y))
                }
            }
            6 => Ok(SetRegister(raw.hex_digit_from_left(1), raw.last_byte())),
            7 => Ok(Add(raw.hex_digit_from_left(1), raw.last_byte())),
            8 => {
                let suffix = raw.hex_digit_from_left(3);
                if suffix > 0xE {
                    error_val
                } else {
                    let (x, y) = raw.middle_digits();
                    match suffix {
                        0 => Ok(Assign(x, y)),
                        1 => Ok(AssignOr(x, y)),
                        2 => Ok(AssignAnd(x, y)),
                        3 => Ok(AssignXor(x, y)),
                        4 => Ok(AddAssign(x, y)),
                        5 => Ok(SubAssign(x, y)),
                        6 => Ok(ShiftRight(x)),
                        7 => Ok(FlippedSubAssign(x, y)),
                        0xE => Ok(ShiftLeft(x)),
                        _ => error_val,
                    }
                }
            }
            9 => {
                if raw.hex_digit_from_left(3) == 0 {
                    let (x, y) = raw.middle_digits();
                    Ok(Opcode::SkipIfMismatchReg(x, y))
                } else {
                    error_val
                }
            }
            0xA => Ok(SetIdx(raw.last_three_digits())),
            0xB => Ok(JumpTo(raw.last_three_digits())),
            0xC => Ok(Rand(raw.hex_digit_from_left(1), raw.last_byte())),
            0xD => Ok(Draw(
                raw.hex_digit_from_left(1),
                raw.hex_digit_from_left(2),
                raw.hex_digit_from_left(3),
            )),
            0xE => {
                if raw.hex_digit_from_left(2) == 9 && raw.hex_digit_from_left(3) == 0xE {
                    Ok(SkipIfPressed(raw.hex_digit_from_left(1)))
                } else if raw.hex_digit_from_left(2) == 0xA && raw.hex_digit_from_left(3) == 1 {
                    Ok(SkipIfNotPressed(raw.hex_digit_from_left(1)))
                } else {
                    error_val
                }
            }
            0xF => {
                let r = raw.hex_digit_from_left(1);
                match (raw.hex_digit_from_left(2), raw.hex_digit_from_left(3)) {
                    (0, 7) => Ok(StoreDelay(r)),
                    (0, 0xA) => Ok(WaitKey),
                    (1, 5) => Ok(SetDelay(r)),
                    (1, 8) => Ok(SetSound(r)),
                    (1, 0xE) => Ok(IncrementIdx(r)),
                    (2, 9) => Ok(NewSprite(r)),
                    (3, 3) => Ok(BCD(r)),
                    (5, 5) => Ok(DumpRegisters(r)),
                    (6, 5) => Ok(FillRegisters(r)),
                    (_, _) => error_val,
                }
            }
            _ => error_val,
        }
    }
}

impl TryFrom<u16> for Opcode {
    type Error = anyhow::Error;
    fn try_from(x: u16) -> Result<Self, Self::Error> {
        Ok(Self::try_from(RawOpcode::try_from(x)?)?)
    }
}
