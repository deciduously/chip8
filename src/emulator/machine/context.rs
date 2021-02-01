//! This is the interface for a machine to interact with the outside
use super::*;

#[cfg(feature = "sdl")]
pub use sdl::SdlContext;

#[cfg(test)]
pub use test_context::TestContext;

pub trait Context {
    /// Call once to initalize systems and prepare to loop
    fn init(&mut self);
    /// CGaather iniput for the tick, return true if user requested a quit
    fn listen_for_input(&mut self) -> bool;
    /// Draw the current stored screen state out to the real screen
    fn draw_graphics(&mut self, screen: Screen);
    /// Retreive the current real-world key state
    fn get_key_state(&self) -> Keys;
    /// Get a random byte
    fn random_byte(&self) -> u8;
}

#[cfg(feature = "sdl")]
mod sdl {
    use super::*;
    use sdl2::{
        self, event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::WindowCanvas,
        EventPump,
    };

    /// Helper to converd an SDL Keycode to a normalized keypad value to store
    fn keycode_to_keypad(keycode: Option<Keycode>) -> Option<u8> {
        // First, make sure it's not a None
        keycode?;

        use Keycode::*;
        let c = match keycode.unwrap() {
            Num1 => '1',
            Num2 => '2',
            Num3 => '3',
            Num4 => '4',
            Q => 'Q',
            W => 'W',
            E => 'E',
            R => 'R',
            A => 'A',
            S => 'S',
            D => 'D',
            F => 'F',
            Z => 'Z',
            X => 'X',
            C => 'C',
            V => 'V',
            _ => '.', // this will turn into an error, end then a None
        };
        keyboard_to_keypad(c).ok()
    }

    /// Sdl2 context
    pub struct SdlContext {
        canvas: WindowCanvas,
        event_pump: EventPump,
        key_state: Keys,
    }

    impl SdlContext {
        pub fn new(scale_factor: u8) -> Box<Self> {
            let scale_factor = scale_factor as u32;
            let window_width = PIXEL_COLS * scale_factor;
            let window_height = PIXEL_ROWS * scale_factor;

            let context = sdl2::init().unwrap();
            let video_subsystem = context.video().unwrap();
            let window = video_subsystem
                .window("CHIP 8 - SDL2 Renderer", window_width, window_height)
                .position_centered()
                .build()
                .unwrap();
            let mut canvas = window
                .into_canvas()
                .build()
                .map_err(|e| e.to_string())
                .unwrap();
            let event_pump = context.event_pump().unwrap();

            let scale_factor = scale_factor as f32;
            canvas.set_scale(scale_factor, scale_factor).unwrap();

            let ret = Self {
                canvas,
                event_pump,
                key_state: FRESH_KEYS,
            };

            Box::new(ret)
        }
    }

    impl Context for SdlContext {
        fn init(&mut self) {
            self.canvas.clear();
            self.canvas.present();
        }
        fn listen_for_input(&mut self) -> bool {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => return true,
                    Event::KeyDown { keycode, .. } => {
                        if let Some(key_pressed) = keycode_to_keypad(keycode) {
                            self.key_state[key_pressed as usize] = true;
                        }
                    }
                    Event::KeyUp { keycode, .. } => {
                        if let Some(key_pressed) = keycode_to_keypad(keycode) {
                            self.key_state[key_pressed as usize] = false;
                        }
                    }
                    _ => {}
                }
            }
            false
        }

        fn draw_graphics(&mut self, screen: Screen) {
            // For each pixel in the screen, draw a filled rectangle

            // First, clear the canvas to black.
            self.canvas.set_draw_color(Color::RGB(0, 0, 0));
            self.canvas.clear();
            // Set to white to draw pixels.
            self.canvas.set_draw_color(Color::RGB(255, 255, 255));

            for y in 0..PIXEL_ROWS {
                for x in 0..PIXEL_COLS {
                    // Draw a point if it exists scaled up from the source screen
                    if screen[(x + (y * PIXEL_COLS)) as usize] == 1 {
                        self.canvas
                            .fill_rect(Rect::new(x as i32, y as i32, 1, 1))
                            .unwrap();
                    }
                }
            }

            self.canvas.present();
        }

        fn get_key_state(&self) -> Keys {
            self.key_state
        }
        
        fn random_byte(&self) -> u8 {
            rand::random::<u8>()
        }
    }
}

#[cfg(wasm)]
mod wasm_context {
    // Helper macros for DOM manipulation
    macro_rules! append_attrs {
        ($document:ident, $el:ident, $( $attr:expr ),* ) => {
            $(
                let attr = $document.create_attribute($attr.0)?;
                attr.set_value($attr.1);
                $el.set_attribute_node(&attr)?;
            )*
        }
    }
    
    macro_rules! append_text_child {
        ($document:ident, $el:ident, $text:expr ) => {
            let text = $document.create_text_node($text);
            $el.append_child(&text)?;
        };
    }
    
    macro_rules! create_element_attrs {
        ($document:ident, $type:expr, $( $attr:expr ),* ) => {{
            let el = $document.create_element($type)?;
            append_attrs!($document, el, $( $attr ),*);
            el}
        }
    }
    
    macro_rules! append_element_attrs {
        ($document:ident, $parent:ident, $type:expr, $( $attr:expr ),* ) => {
            let el = create_element_attrs!($document, $type, $( $attr ),* );
            $parent.append_child(&el)?;
        }
    }
    
    macro_rules! append_text_element_attrs {
        ($document:ident, $parent:ident, $type:expr, $text:expr, $( $attr:expr ),*) => {
            let el = create_element_attrs!($document, $type, $( $attr ),* );
            append_text_child!($document, el, $text);
            $parent.append_child(&el)?;
        }
    }
    
    /// The WebAssembly interface
    #[derive(Debug)]
    pub struct WasmContext;

    impl WasmContext {
        pub fn new() -> Self {
            // Create the page and such, store a reference to the canvas.
            
            todo!()
        }
    }

    impl Context for WasmContext {
        fn init(&mut self) {}
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
    }

    #[wasm_bindgen]
    fn run() -> Result<()> {
        // get document, get body, init machine, etd
        // this is the WASM "main"
        todo!()
    }
}

#[cfg(test)]
mod test_context {
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
    }
}
