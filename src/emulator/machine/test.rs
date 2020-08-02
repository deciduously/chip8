use super::*;
use pretty_assertions::assert_eq;
use std::convert::TryFrom;

#[test]
fn test_load_fonts() {
    let machine = Machine::new();
    // The constructor should properly load the full fontset and nothing else.
    assert_eq!(machine.memory_get(0), 0xF0);
    assert_eq!(machine.memory_get(79), 0x80);
    assert_eq!(machine.memory_get(80), 0);
}

#[test]
fn test_load_game() {
    let mut machine = Machine::new();
    let bytes = machine.load_game("pong").unwrap();
    assert_eq!(bytes, 246);
    assert_eq!(machine.current_byte(), 0x6A)
}

#[test]
fn test_load_second_game() {
    // Should clear memory and load the new game
    let mut machine = Machine::new();
    let _: usize = machine.load_game("pong").unwrap();
    let bytes = machine.load_game("tank").unwrap();
    assert_eq!(bytes, 560);
    assert_eq!(machine.current_byte(), 0x12)
}

#[test]
fn test_game_not_found() {
    let mut machine = Machine::new();
    assert_eq!(
        machine.load_game("ping").err().unwrap().to_string(),
        "No such file or directory (os error 2)".to_string()
    );
}

#[test]
fn test_opcode_2nnn_call() {
    let mut machine = Machine::new();
    Opcode::try_from(0x2BCD).unwrap().execute(&mut machine);
    // Should store the current location in the stack to jump back later
    assert_eq!(machine.stack[0], PC_BEGIN as usize);
    // Should increment stack pointer
    assert_eq!(machine.sp, 1);
    // Should set program counter to new location
    assert_eq!(machine.pc, 0xBCD);
}

#[test]
fn test_opcode_6xnn_set_register() {
    let mut machine = Machine::new();
    // Seed registers
    machine.registers[0xB] = 3;
    Opcode::try_from(0x6BCD).unwrap().execute(&mut machine);

    // Should set register to given value
    assert_eq!(machine.register_get(0xB), 0xCD);
    // Should increment program counter by two
    assert_eq!(machine.pc, PC_BEGIN + 2);
}

#[test]
fn test_opcode_7xnn_add() {
    let mut machine = Machine::new();
    // Seed registers
    machine.registers[0xB] = 3;
    Opcode::try_from(0x7BCD).unwrap().execute(&mut machine);

    // Should add NN to reg_x
    assert_eq!(machine.register_get(0xB), 3 + 0xCD);
    // Should increment program counter by two
    assert_eq!(machine.pc, PC_BEGIN + 2);
}

#[test]
fn test_opcode_8xy0_assign() {
    let mut machine = Machine::new();
    // Seed registers
    machine.registers[0xB] = 3;
    machine.registers[0xC] = 15;
    Opcode::try_from(0x8BC0).unwrap().execute(&mut machine);

    // Should assign VY to VX
    assert_eq!(machine.register_get(0xB), 15);
    // Should not affect VY
    assert_eq!(machine.register_get(0xC), 15);
    // Should increment program counter by two
    assert_eq!(machine.pc, PC_BEGIN + 2);
}

#[test]
fn test_opcode_8xy1_assign_or() {
    let mut machine = Machine::new();
    // Seed registers
    machine.registers[0xB] = 0xA;
    machine.registers[0xC] = 4;
    Opcode::try_from(0x8BC1).unwrap().execute(&mut machine);

    // Should assign VY to (VX | VY)
    assert_eq!(machine.register_get(0xB), 14);
    // Should not affect VY
    assert_eq!(machine.register_get(0xC), 4);
    // Should increment program counter by two
    assert_eq!(machine.pc, PC_BEGIN + 2);
}

#[test]
fn test_opcode_8xy2_assign_and() {
    let mut machine = Machine::new();
    // Seed registers
    machine.registers[0xB] = 0xA;
    machine.registers[0xC] = 0xC;
    Opcode::try_from(0x8BC2).unwrap().execute(&mut machine);

    // Should assign VY to (VX & VY)
    assert_eq!(machine.register_get(0xB), 8);
    // Should not affect VY
    assert_eq!(machine.register_get(0xC), 0xC);
    // Should increment program counter by two
    assert_eq!(machine.pc, PC_BEGIN + 2);
}

#[test]
fn test_opcode_8xy3_assign_xor() {
    let mut machine = Machine::new();
    // Seed registers
    machine.registers[0xB] = 0xA;
    machine.registers[0xC] = 0xC;
    Opcode::try_from(0x8BC3).unwrap().execute(&mut machine);

    // Should assign VY to (VX ^ VY)
    assert_eq!(machine.register_get(0xB), 6);
    // Should not affect VY
    assert_eq!(machine.register_get(0xC), 0xC);
    // Should increment program counter by two
    assert_eq!(machine.pc, PC_BEGIN + 2);
}

#[test]
fn test_opcode_8xy4_add_assign() {
    let mut machine = Machine::new();
    // Seed registers
    machine.registers[0xB] = 3;
    machine.registers[0xC] = 15;
    Opcode::try_from(0x8BC4).unwrap().execute(&mut machine);

    // Should add VY to VX, wrapping around 0xFF
    assert_eq!(machine.register_get(0xB), 18);
    // Should not affect VY
    assert_eq!(machine.register_get(0xC), 15);
    // Should not set carry flag
    assert!(!machine.carry_flag_set());
    // Should increment program counter by two
    assert_eq!(machine.pc, PC_BEGIN + 2);
}

#[test]
fn test_opcode_8xy4_add_assign_with_carry() {
    let mut machine = Machine::new();
    // Seed registers - each is only one byte, so this will wrap over
    machine.registers[0xB] = 250;
    machine.registers[0xC] = 15;
    Opcode::try_from(0x8BC4).unwrap().execute(&mut machine);

    // Should add VY to VX, wrapping around 0xFF
    assert_eq!(machine.register_get(0xB), 10);
    // Should not affect VY
    assert_eq!(machine.register_get(0xC), 15);
    // Should set carry flag
    assert!(machine.carry_flag_set());
    // Should increment program counter by two
    assert_eq!(machine.pc, PC_BEGIN + 2);
}

#[test]
fn test_opcode_8xy5_sub_assign() {
    let mut machine = Machine::new();
    // Seed registers
    machine.registers[0xB] = 0xD;
    machine.registers[0xC] = 0xA;
    Opcode::try_from(0x8BC5).unwrap().execute(&mut machine);

    // Should subtract VY from VX, wrapping around 0xFF
    assert_eq!(machine.register_get(0xB), 3);
    // Should not affect VY
    assert_eq!(machine.register_get(0xC), 0xA);
    // Should set carry flag (in this case, it means no borrow)
    assert!(machine.carry_flag_set());
    // Should increment program counter by two
    assert_eq!(machine.pc, PC_BEGIN + 2);
}

#[test]
fn test_opcode_8xy5_sub_assign_with_borrow() {
    let mut machine = Machine::new();
    // Seed registers - each is only one byte, so this will wrap over
    machine.registers[0xB] = 0xA;
    machine.registers[0xC] = 0xD;
    Opcode::try_from(0x8BC5).unwrap().execute(&mut machine);

    // Should subtract VY from VX, wrapping around 0xFF
    assert_eq!(machine.register_get(0xB), 0xFD);
    // Should not affect VY
    assert_eq!(machine.register_get(0xC), 0xD);
    // Should not set carry flag (in this case, it means there was a borrow)
    assert!(!machine.carry_flag_set());
    // Should increment program counter by two
    assert_eq!(machine.pc, PC_BEGIN + 2);
}

#[test]
fn test_opcode_8xy6_shift_right() {
    let mut machine = Machine::new();
    // Seed registers - each is only one byte, so this will wrap over
    machine.registers[0xB] = 0xA;
    machine.registers[0xC] = 0xD;
    Opcode::try_from(0x8BC6).unwrap().execute(&mut machine);

    // Should store least significant bit of VX as the borrow flag
    assert_eq!(machine.register_get(0xF), (0xA & 1));
    // Should shift VX right 1
    assert_eq!(machine.register_get(0xB), (0xA >> 1));
    // Should not affect VY
    assert_eq!(machine.register_get(0xC), 0xD);
    // Should increment program counter by two
    assert_eq!(machine.pc, PC_BEGIN + 2);
}

#[test]
fn test_opcode_8xy7_flipped_sub_assign() {
    let mut machine = Machine::new();
    // Seed registers
    machine.registers[0xB] = 0xA;
    machine.registers[0xC] = 0xD;
    Opcode::try_from(0x8BC7).unwrap().execute(&mut machine);

    // Should subtract VY from VX, wrapping around 0xFF
    assert_eq!(machine.register_get(0xB), 3);
    // Should not affect VY
    assert_eq!(machine.register_get(0xC), 0xD);
    // Should set carry flag (in this case, it means no borrow)
    assert!(machine.carry_flag_set());
    // Should increment program counter by two
    assert_eq!(machine.pc, PC_BEGIN + 2);
}

#[test]
fn test_opcode_8xy7_flipped_sub_assign_with_borrow() {
    let mut machine = Machine::new();
    // Seed registers - each is only one byte, so this will wrap over
    machine.registers[0xB] = 0xD;
    machine.registers[0xC] = 2;
    Opcode::try_from(0x8BC7).unwrap().execute(&mut machine);

    // Should subtract VY from VX, wrapping around 0xFF
    assert_eq!(machine.register_get(0xB), 0xF5);
    // Should not affect VY
    assert_eq!(machine.register_get(0xC), 2);
    // Should not set carry flag (in this case, it means there was a borrow)
    assert!(!machine.carry_flag_set());
    // Should increment program counter by two
    assert_eq!(machine.pc, PC_BEGIN + 2);
}

#[test]
fn test_opcode_8xye_shift_left() {
    let mut machine = Machine::new();
    // Seed registers - each is only one byte, so this will wrap over
    machine.registers[0xB] = 0xA;
    machine.registers[0xC] = 0xD;
    Opcode::try_from(0x8BCE).unwrap().execute(&mut machine);

    // Should store most significant bit of VX as the borrow flag
    assert_eq!(machine.register_get(0xF), (0xA >> (8 - 1) & 1));
    // Should shift VX left 1
    assert_eq!(machine.register_get(0xB), (0xA << 1));
    // Should not affect VY
    assert_eq!(machine.register_get(0xC), 0xD);
    // Should increment program counter by two
    assert_eq!(machine.pc, PC_BEGIN + 2);
}

#[test]
fn test_opcode_annn_set_idx() {
    let mut machine = Machine::new();
    Opcode::try_from(0xABCD).unwrap().execute(&mut machine);
    // Should store index given
    assert_eq!(machine.idx, 0xBCD);
    // Should increment program counter by two
    assert_eq!(machine.pc, PC_BEGIN + 2);
}

#[test]
fn test_opcode_bnnn_jump_to() {
    let mut machine = Machine::new();
    machine.registers[0] = 4;
    Opcode::try_from(0xBCDE).unwrap().execute(&mut machine);
    // Should advance to NNN plus V0
    assert_eq!(machine.pc, 0xCDE + 4);
}

#[test]
fn test_opcode_cxnn_rand() {
    let mut machine = Machine::new();
    machine.registers[0xA] = 4;
    Opcode::try_from(0xCABA).unwrap().execute(&mut machine);
    // Should have a new value in VX (even though it might sometimes end up the same :/)
    // FIXME SeedableRng is what you want to do, probably
    assert!(machine.register_get(0xA) != 4);
    // Should increment program counter by two
    assert_eq!(machine.pc, PC_BEGIN + 2);
}

#[test]
fn test_opcode_fx07_store_delay() {
    let mut machine = Machine::new();
    machine.registers[0xA] = 4;
    Opcode::try_from(0xFA07).unwrap().execute(&mut machine);
    // Should store the delay timer value in VX, which hasn't changed from the max since instatiation
    assert_eq!(machine.register_get(0xA), 0xFF);
    // Should increment program counter by two
    assert_eq!(machine.pc, PC_BEGIN + 2);
}

#[test]
fn test_opcode_fx15_set_delay() {
    let mut machine = Machine::new();
    machine.registers[0xA] = 4;
    Opcode::try_from(0xFA15).unwrap().execute(&mut machine);
    // Should set the delay timer value in VX
    assert_eq!(machine.delay_timer, 4);
    // Should increment program counter by two
    assert_eq!(machine.pc, PC_BEGIN + 2);
}

#[test]
fn test_opcode_fx18_set_sound() {
    let mut machine = Machine::new();
    machine.registers[0xA] = 4;
    Opcode::try_from(0xFA18).unwrap().execute(&mut machine);
    // Should set the sound timer value in VX
    assert_eq!(machine.sound_timer, 4);
    // Should increment program counter by two
    assert_eq!(machine.pc, PC_BEGIN + 2);
}

#[test]
fn test_opcode_fx33_bcd() {
    let mut machine = Machine::new();
    machine.registers[0xB] = 195;
    machine.idx = 0xAB;
    Opcode::try_from(0xFB33).unwrap().execute(&mut machine);
    // Should store the BCD of V[X] to the right memory locations
    assert_eq!(machine.memory[0xAB], 1);
    assert_eq!(machine.memory[0xAB + 1], 9);
    assert_eq!(machine.memory[0xAB + 2], 5);
    // Should increment program counter by two
    assert_eq!(machine.pc, PC_BEGIN + 2);
}

#[test]
fn test_fx55_dump_registers() {
    let mut machine = Machine::new();
    machine.idx = 0xBCD;
    machine.registers[0] = 0xC;
    machine.registers[1] = 4;
    machine.registers[2] = 123;
    machine.registers[3] = 98;
    machine.registers[4] = 12;
    Opcode::try_from(0xF355).unwrap().execute(&mut machine);
    // Should store V0 thorugh VX inclusive to memory starting at the index pointer
    let i = machine.idx as usize;
    assert_eq!(machine.memory[i], 0xC);
    assert_eq!(machine.memory[i + 1], 4);
    assert_eq!(machine.memory[i + 2], 123);
    assert_eq!(machine.memory[i + 3], 98);
    // Should only affect that memory, no further
    assert_eq!(machine.memory[i + 4], 0);
    // SHould not modify the index pointer
    assert_eq!(machine.idx, 0xBCD);
    // Should not modify the registers
    assert_eq!(machine.register_get(0), 0xC);
    assert_eq!(machine.register_get(1), 4);
    assert_eq!(machine.register_get(2), 123);
    assert_eq!(machine.register_get(3), 98);
    assert_eq!(machine.register_get(4), 12);
    // Should increment program counter by two
    assert_eq!(machine.pc, PC_BEGIN + 2);
}

#[test]
fn test_fx65_fill_registers() {
    let mut machine = Machine::new();
    machine.idx = 0xBCD;
    machine.memory[0xBCD] = 0xC;
    machine.memory[0xBCD + 1] = 4;
    machine.memory[0xBCD + 2] = 123;
    machine.memory[0xBCD + 3] = 98;
    machine.memory[0xBCD + 4] = 12;
    Opcode::try_from(0xF365).unwrap().execute(&mut machine);
    // Should not touch the memory
    let i = machine.idx as usize;
    assert_eq!(machine.memory[i], 0xC);
    assert_eq!(machine.memory[i + 1], 4);
    assert_eq!(machine.memory[i + 2], 123);
    assert_eq!(machine.memory[i + 3], 98);
    assert_eq!(machine.memory[i + 4], 12);
    assert_eq!(machine.memory[i + 5], 0);
    // SHould not modify the index pointer
    assert_eq!(machine.idx, 0xBCD);
    // Should store the registers
    assert_eq!(machine.register_get(0), 0xC);
    assert_eq!(machine.register_get(1), 4);
    assert_eq!(machine.register_get(2), 123);
    assert_eq!(machine.register_get(3), 98);
    assert_eq!(machine.register_get(4), 0);
    // Should increment program counter by two
    assert_eq!(machine.pc, PC_BEGIN + 2);
}