use super::*;
use pretty_assertions::assert_eq;

#[test]
fn test_load_fonts() {
    let machine = Machine::new(TestContext::new());
    // The constructor should properly load the full fontset and nothing else.
    assert_eq!(machine.memory_get(0), 0xF0);
    assert_eq!(machine.memory_get(79), 0x80);
    assert_eq!(machine.memory_get(80), 0);
}

#[test]
fn test_load_game() {
    let mut machine = Machine::new(TestContext::new());
    let bytes = machine.load_game("pong").unwrap();
    assert_eq!(bytes, 246);
    assert_eq!(machine.current_byte(), 0x6A)
}

#[test]
fn test_load_second_game() {
    // Should clear memory and load the new game
    let mut machine = Machine::new(TestContext::new());
    let _: usize = machine.load_game("pong").unwrap();
    let bytes = machine.load_game("tank").unwrap();
    assert_eq!(bytes, 560);
    assert_eq!(machine.current_byte(), 0x12)
}

#[test]
fn test_game_not_found() {
    let mut machine = Machine::new(TestContext::new());
    assert_eq!(
        machine.load_game("ping").err().unwrap().to_string(),
        "Game ping not included".to_string()
    );
}

#[test]
fn test_1nnn_jump() {
    let mut machine = Machine::new(TestContext::new());
    machine.test_opcode(0x1CDE);
    // Should advance to NNN
    assert_eq!(machine.pc, 0xCDE);
}

#[test]
fn test_2nnn_call() {
    let mut machine = Machine::new(TestContext::new());
    machine.test_opcode(0x2BCD);
    // Should store the current location in the stack to jump back later
    assert_eq!(machine.stack[0], PC_BEGIN);
    // Should increment stack pointer
    assert_eq!(machine.sp, 1);
    // Should set program counter to new location
    assert_eq!(machine.pc, 0xBCD);
}

#[test]
fn test_00ee_return() {
    let mut machine = Machine::new(TestContext::new());
    machine.test_opcode(0x2BCD); // Call at 0xBCD
    machine.test_opcode(0x00EE); // Return to start
                                 // Should clear the call stack
    assert_eq!(machine.stack[0], 0);
    // Should decrement stack pointer
    assert_eq!(machine.sp, 0);
    // Should set program counter back to original location
    assert_eq!(machine.pc, PC_BEGIN + 2);
}

#[test]
fn test_3xnn_skip_if_eq_val() {
    let mut machine = Machine::new(TestContext::new());
    machine.register_set(0xB, 0xCD);
    machine.test_opcode(0x3BCD);
    // Should skip next instruction
    assert_eq!(machine.pc, PC_BEGIN + 4);

    // Should not skip if there is no match
    machine.reset();
    machine.register_set(0xB, 2);
    machine.test_opcode(0x3BCD);
    // Should skip next instruction
    assert_eq!(machine.pc, PC_BEGIN + 2);
}

#[test]
fn test_4xnn_skip_if_not_eq_val() {
    let mut machine = Machine::new(TestContext::new());
    machine.register_set(0xB, 2);
    machine.test_opcode(0x4BCD);
    // Should skip next instruction
    assert_eq!(machine.pc, PC_BEGIN + 4);

    // Should not skip if there is no match
    machine.reset();
    machine.register_set(0xB, 0xCD);
    machine.test_opcode(0x4BCD);
    // Should skip next instruction
    assert_eq!(machine.pc, PC_BEGIN + 2);
}

#[test]
fn test_5xy0_skip_if_match_reg() {
    let mut machine = Machine::new(TestContext::new());
    machine.register_set(0xB, 0xCD);
    machine.register_set(0xC, 0xCD);
    machine.test_opcode(0x5BC0);
    // Should skip next instruction
    assert_eq!(machine.pc, PC_BEGIN + 4);

    // Should not skip if there is no match
    machine.reset();
    machine.register_set(0xB, 0xCD);
    machine.register_set(0xC, 3);
    machine.test_opcode(0x5BC0);
    // Should skip next instruction
    assert_eq!(machine.pc, PC_BEGIN + 2);
}

#[test]
fn test_6xnn_set_register() {
    let mut machine = Machine::new(TestContext::new());
    // Seed registers
    machine.register_set(0xB, 3);
    machine.test_opcode(0x6BCD);

    // Should set register to given value
    assert_eq!(machine.register_get(0xB), 0xCD);
    // Should increment program counter by two
    assert_eq!(machine.pc, PC_BEGIN + 2);
}

#[test]
fn test_7xnn_add() {
    let mut machine = Machine::new(TestContext::new());
    // Seed registers
    machine.register_set(0xB, 3);
    machine.test_opcode(0x7BCD);

    // Should add NN to reg_x
    assert_eq!(machine.register_get(0xB), 3 + 0xCD);
    // Should increment program counter by two
    assert_eq!(machine.pc, PC_BEGIN + 2);
}

#[test]
fn test_8xy0_assign() {
    let mut machine = Machine::new(TestContext::new());
    // Seed registers
    machine.register_set(0xB, 3);
    machine.register_set(0xC, 15);
    machine.test_opcode(0x8BC0);

    // Should assign VY to VX
    assert_eq!(machine.register_get(0xB), 15);
    // Should not affect VY
    assert_eq!(machine.register_get(0xC), 15);
    // Should increment program counter by two
    assert_eq!(machine.pc, PC_BEGIN + 2);
}
#[test]
fn test_8xy1_assign_or() {
    let mut machine = Machine::new(TestContext::new());
    // Seed registers
    machine.register_set(0xB, 0xA);
    machine.register_set(0xC, 4);
    machine.test_opcode(0x8BC1);

    // Should assign VY to (VX | VY)
    assert_eq!(machine.register_get(0xB), 14);
    // Should not affect VY
    assert_eq!(machine.register_get(0xC), 4);
    // Should increment program counter by two
    assert_eq!(machine.pc, PC_BEGIN + 2);
}

#[test]
fn test_8xy2_assign_and() {
    let mut machine = Machine::new(TestContext::new());
    // Seed registers
    machine.register_set(0xB, 0xA);
    machine.register_set(0xC, 0xC);
    machine.test_opcode(0x8BC2);

    // Should assign VY to (VX & VY)
    assert_eq!(machine.register_get(0xB), 8);
    // Should not affect VY
    assert_eq!(machine.register_get(0xC), 0xC);
    // Should increment program counter by two
    assert_eq!(machine.pc, PC_BEGIN + 2);
}

#[test]
fn test_8xy3_assign_xor() {
    let mut machine = Machine::new(TestContext::new());
    // Seed registers
    machine.register_set(0xB, 0xA);
    machine.register_set(0xC, 0xC);
    machine.test_opcode(0x8BC3);

    // Should assign VY to (VX ^ VY)
    assert_eq!(machine.register_get(0xB), 6);
    // Should not affect VY
    assert_eq!(machine.register_get(0xC), 0xC);
    // Should increment program counter by two
    assert_eq!(machine.pc, PC_BEGIN + 2);
}

#[test]
fn test_8xy4_add_assign() {
    let mut machine = Machine::new(TestContext::new());
    // Seed registers
    machine.register_set(0xB, 3);
    machine.register_set(0xC, 15);
    machine.test_opcode(0x8BC4);

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
fn test_8xy4_add_assign_with_carry() {
    let mut machine = Machine::new(TestContext::new());
    // Seed registers - each is only one byte, so this will wrap over
    machine.register_set(0xB, 250);
    machine.register_set(0xC, 15);
    machine.test_opcode(0x8BC4);

    // Should add VY to VX, wrapping around 0xFF
    assert_eq!(machine.register_get(0xB), 250u8.wrapping_add(15));
    // Should not affect VY
    assert_eq!(machine.register_get(0xC), 15);
    // Should set carry flag
    assert!(machine.carry_flag_set());
    // Should increment program counter by two
    assert_eq!(machine.pc, PC_BEGIN + 2);
}

#[test]
fn test_8xy5_sub_assign() {
    let mut machine = Machine::new(TestContext::new());
    // Seed registers
    machine.register_set(0xB, 0xD);
    machine.register_set(0xC, 0xA);
    machine.test_opcode(0x8BC5);

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
fn test_8xy5_sub_assign_with_borrow() {
    let mut machine = Machine::new(TestContext::new());
    // Seed registers - each is only one byte, so this will wrap over
    machine.register_set(0xB, 0xA);
    machine.register_set(0xC, 0xD);
    machine.test_opcode(0x8BC5);

    // Should subtract VY from VX, wrapping around 0xFF
    assert_eq!(machine.register_get(0xB), 0xAu8.wrapping_sub(0xD));
    // Should not affect VY
    assert_eq!(machine.register_get(0xC), 0xD);
    // Should not set carry flag (in this case, it means there was a borrow)
    assert!(!machine.carry_flag_set());
    // Should increment program counter by two
    assert_eq!(machine.pc, PC_BEGIN + 2);
}

#[test]
fn test_8xy6_shift_right() {
    let mut machine = Machine::new(TestContext::new());
    // Seed registers - each is only one byte, so this will wrap over
    machine.register_set(0xB, 0xA);
    machine.register_set(0xC, 0xD);
    machine.test_opcode(0x8BC6);

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
fn test_8xy7_flipped_sub_assign() {
    let mut machine = Machine::new(TestContext::new());
    // Seed registers
    machine.register_set(0xB, 0xA);
    machine.register_set(0xC, 0xD);
    machine.test_opcode(0x8BC7);
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
fn test_8xy7_flipped_sub_assign_with_borrow() {
    let mut machine = Machine::new(TestContext::new());
    // Seed registers - each is only one byte, so this will wrap over
    machine.register_set(0xB, 0xD);
    machine.register_set(0xC, 2);
    machine.test_opcode(0x8BC7);

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
fn test_8xye_shift_left() {
    let mut machine = Machine::new(TestContext::new());
    // Seed registers - each is only one byte, so this will wrap over
    machine.register_set(0xB, 0xA);
    machine.register_set(0xC, 0xD);
    machine.test_opcode(0x8BCE);

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
fn test_annn_set_idx() {
    let mut machine = Machine::new(TestContext::new());
    machine.test_opcode(0xABCD);
    // Should store index given
    assert_eq!(machine.idx, 0xBCD);
    // Should increment program counter by two
    assert_eq!(machine.pc, PC_BEGIN + 2);
}

#[test]
fn test_bnnn_jump_to() {
    let mut machine = Machine::new(TestContext::new());
    machine.register_set(0, 4);
    machine.test_opcode(0xBCDE);
    // Should advance to NNN plus V0
    assert_eq!(machine.pc, 0xCDE + 4);
}

#[test]
fn test_cxnn_rand() {
    let mut machine = Machine::new(TestContext::new());
    machine.register_set(0xA, 4);
    machine.test_opcode(0xCABA);
    // Should have a new value in VX (even though it might sometimes end up the same :/)
    // FIXME SeedableRng is what you want to do, probably
    assert!(machine.register_get(0xA) != 4);
    // Should increment program counter by two
    assert_eq!(machine.pc, PC_BEGIN + 2);
}

#[test]
fn test_fx07_store_delay() {
    let mut machine = Machine::new(TestContext::new());
    machine.register_set(0xA, 4);
    machine.test_opcode(0xFA07);
    // Should store the delay timer value in VX, which hasn't changed from the max since instatiation
    assert_eq!(machine.register_get(0xA), 0xFF);
    // Should increment program counter by two
    assert_eq!(machine.pc, PC_BEGIN + 2);
}

#[test]
fn test_fx15_set_delay() {
    let mut machine = Machine::new(TestContext::new());
    machine.register_set(0xA, 4);
    machine.test_opcode(0xFA15);
    // Should set the delay timer value in VX
    assert_eq!(machine.delay_timer, 4);
    // Should increment program counter by two
    assert_eq!(machine.pc, PC_BEGIN + 2);
}

#[test]
fn test_fx18_set_sound() {
    let mut machine = Machine::new(TestContext::new());
    machine.register_set(0xA, 4);
    machine.test_opcode(0xFA18);
    // Should set the sound timer value in VX
    assert_eq!(machine.sound_timer, 4);
    // Should increment program counter by two
    assert_eq!(machine.pc, PC_BEGIN + 2);
}

#[test]
fn test_fx1e_increment_idx() {
    let mut machine = Machine::new(TestContext::new());
    machine.idx = 0xBCD;
    machine.register_set(0xA, 4);
    machine.test_opcode(0xFA1E);
    // Should increment the index pointer by the value set at VX
    assert_eq!(machine.idx, 0xBCD + 4);
    // Should not touch VX
    assert_eq!(machine.register_get(0xA), 4);
    // Should increment program counter by two
    assert_eq!(machine.pc, PC_BEGIN + 2);
}

#[test]
fn test_fx33_bcd() {
    let mut machine = Machine::new(TestContext::new());
    machine.register_set(0xB, 195);
    machine.idx = 0xAB;
    machine.test_opcode(0xFB33);
    // Should store the BCD of V[X] to the right memory locations
    assert_eq!(machine.memory_get(0xAB), 1);
    assert_eq!(machine.memory_get(0xAB + 1), 9);
    assert_eq!(machine.memory_get(0xAB + 2), 5);
    // Should increment program counter by two
    assert_eq!(machine.pc, PC_BEGIN + 2);
}

#[test]
fn test_fx55_dump_registers() {
    let mut machine = Machine::new(TestContext::new());
    machine.idx = 0xBCD;
    machine.register_set(0, 0xC);
    machine.register_set(1, 4);
    machine.register_set(2, 123);
    machine.register_set(3, 98);
    machine.register_set(4, 12);
    machine.test_opcode(0xF355);
    // Should store V0 thorugh VX inclusive to memory starting at the index pointer
    let i = machine.idx;
    assert_eq!(machine.memory_get(i), 0xC);
    assert_eq!(machine.memory_get(i + 1), 4);
    assert_eq!(machine.memory_get(i + 2), 123);
    assert_eq!(machine.memory_get(i + 3), 98);
    // Should only affect that memory, no further
    assert_eq!(machine.memory_get(i + 4), 0);
    // Should not modify the index pointer
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
    let mut machine = Machine::new(TestContext::new());
    machine.idx = 0xBCD;
    machine.memory_set(0xBCD, 0xC);
    machine.memory_set(0xBCD + 1, 4);
    machine.memory_set(0xBCD + 2, 123);
    machine.memory_set(0xBCD + 3, 98);
    machine.memory_set(0xBCD + 4, 12);
    machine.test_opcode(0xF365);
    // Should not touch the memory
    let i = machine.idx;
    assert_eq!(machine.memory_get(i), 0xC);
    assert_eq!(machine.memory_get(i + 1), 4);
    assert_eq!(machine.memory_get(i + 2), 123);
    assert_eq!(machine.memory_get(i + 3), 98);
    assert_eq!(machine.memory_get(i + 4), 12);
    assert_eq!(machine.memory_get(i + 5), 0);
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

#[test]
fn test_ex9e_skip_if_pressed() {
    let mut machine = Machine::new(TestContext::new());
    machine.register_set(0xB, 0xC);
    machine.press_key(0xC);
    machine.test_opcode(0xEB9E);
    // Should skip next instruction
    assert_eq!(machine.pc, PC_BEGIN + 4);

    // Should not skip if there is no press
    machine.reset();
    machine.register_set(0xB, 0xD);
    machine.press_key(0xC);
    machine.test_opcode(0xEB9E);
    assert_eq!(machine.pc, PC_BEGIN + 2);
}

#[test]
fn test_exa1_skip_if_not_pressed() {
    let mut machine = Machine::new(TestContext::new());
    machine.register_set(0xB, 0xD);
    machine.press_key(0xC);
    machine.test_opcode(0xEBA1);
    // Should skip next instruction
    assert_eq!(machine.pc, PC_BEGIN + 4);

    // Should not skip if it is pressed
    machine.reset();
    machine.register_set(0xB, 0xC);
    machine.press_key(0xC);
    machine.test_opcode(0xEBA1);
    assert_eq!(machine.pc, PC_BEGIN + 2);
}

#[test]
fn test_9xy0_skip_if_mismatch_reg() {
    let mut machine = Machine::new(TestContext::new());
    machine.register_set(0xB, 0xC);
    machine.register_set(0xC, 4);
    machine.test_opcode(0x9BC0);
    // Should skip next instruction
    assert_eq!(machine.pc, PC_BEGIN + 4);

    // Should not skip if they match
    machine.reset();
    machine.register_set(0xB, 0xC);
    machine.register_set(0xC, 0xC);
    machine.test_opcode(0x9BC0);
    assert_eq!(machine.pc, PC_BEGIN + 2);
}

#[test]
fn test_dxyn_draw() {
    let mut machine = Machine::new(TestContext::new());
    machine.memory_set(machine.idx, 0x3C);
    machine.memory_set(machine.idx + 1, 0xC3);
    machine.memory_set(machine.idx + 2, 0xFF);
    machine.test_opcode(0xD003);
    let expected_top = [0, 0, 1, 1, 1, 1, 0, 0];
    let expected_middle = [1, 1, 0, 0, 0, 0, 1, 1];
    let expected_bottom = [1, 1, 1, 1, 1, 1, 1, 1];
    let actual_top = &machine.screen[0..8];
    let actual_middle = &machine.screen[PIXEL_COLS as usize..(PIXEL_COLS + 8) as usize];
    let actual_bottom = &machine.screen[(PIXEL_COLS * 2) as usize..((2 * PIXEL_COLS) + 8) as usize];
    assert_eq!(&expected_top, actual_top);
    assert_eq!(&expected_middle, actual_middle);
    assert_eq!(&expected_bottom, actual_bottom);
}
