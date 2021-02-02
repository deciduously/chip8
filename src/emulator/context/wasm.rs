use super::*;
use wasm_bindgen::prelude::*;
use web_sys::Document;

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
pub struct WasmContext {
    document: Document,
}

impl WasmContext {
    pub fn new() -> Box<Self> {
        // Create the page and such, store a reference to the canvas.
        let window = web_sys::window().expect("Should find window");
        let document = window.document().expect("Should find document");

        Box::new(Self { document })
    }
}

impl Context for WasmContext {
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
}

#[wasm_bindgen]
pub fn run() {
    // get document, get body, init machine, etc
    // this is the WASM "main"
    let context = WasmContext::new();
    let machine = Machine::new(context);
}
