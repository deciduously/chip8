//! A Chip8 VM as a library

pub mod machine;
pub use machine::machine::Machine;

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    #[test]
    fn test_it_compiles() {
        assert_eq!(1 + 1, 2);
    }
}
