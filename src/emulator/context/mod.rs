//! This is the interface for a machine to interact with the outside
use super::machine::*;

#[cfg(feature = "sdl")]
mod sdl;

#[cfg(feature = "wasm")]
pub mod wasm;

#[cfg(feature = "sdl")]
pub use sdl::SdlContext;

#[cfg(test)]
pub use test::TestContext;

/// A Context allows the Machine to interact with a real output screen, speaker, and keyboard.
/// It also handles random number generation.
pub trait Context {
    /// Call once to initalize systems and prepare to loop
    fn init(&mut self);
    /// Produce a beep sound
    fn beep(&self);
    /// CGaather iniput for the tick, return true if user requested a quit
    fn listen_for_input(&mut self) -> bool;
    /// Draw the current stored screen state out to the real screen
    fn draw_graphics(&mut self, screen: Screen);
    /// Retreive the current real-world key state
    fn get_key_state(&self) -> Keys;
    /// Get a random byte
    fn random_byte(&self) -> u8;
    ///// Sleep for a number of milliseconds
    //fn sleep(&self, millis: u64);
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use super::*;

    /// A test context that doesn't actually hook up to anything.
    #[derive(Debug, Default, Clone, Copy)]
    pub struct TestContext;

    impl TestContext {
        pub fn new() -> Box<Self> {
            Box::new(Self::default())
        }
    }

    impl Context for TestContext {
        fn init(&mut self) {}
        fn beep(&self) {}
        fn listen_for_input(&mut self) -> bool {
            false
        }
        fn draw_graphics(&mut self, _screen: Screen) {}
        fn get_key_state(&self) -> Keys {
            FRESH_KEYS
        }
        fn random_byte(&self) -> u8 {
            0x0
        }
        //fn sleep(&self, millis: u64) {
        //    std::thread::sleep(Duration::from_millis(millis));
        //}
    }
}
