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
