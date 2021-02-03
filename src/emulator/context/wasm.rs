use super::*;
use crate::ROMS;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlElement};

type Result<T> = std::result::Result<T, JsValue>;

const INSTRUCTIONS: &str = "Instructions:

Select your preferred game, and use the keys as shown:

  CHIP8   =>  Keyboard

|1|2|3|C| => |1|2|3|4|
|4|5|6|D| => |Q|W|E|R|
|7|8|9|E| => |A|S|D|F|
|A|0|B|F| => |Z|X|C|V|";

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

// Helpers to build the page

fn attach_listener(document: &Document) -> Result<()> {
    // listen for size change events

    update_all()?; // call once for initial render before any changes

    let callback = Closure::wrap(Box::new(move |_evt: web_sys::Event| {
        update_all().expect("Could not update");
    }) as Box<dyn Fn(_)>);

    document
        .get_element_by_id("size")
        .unwrap()
        .dyn_into::<web_sys::HtmlInputElement>()?
        .set_onchange(Some(callback.as_ref().unchecked_ref()));

    callback.forget(); // leaks memory!

    Ok(())
}

fn mount_app(document: &Document, body: &HtmlElement) -> Result<()> {
    append_text_element_attrs!(document, body, "h1", "CHIP-8",);
    mount_controls(&document, &body)?;
    append_text_element_attrs!(document, body, "pre", INSTRUCTIONS,);
    Ok(())
}

fn mount_canvas(document: &Document, parent: &Element) -> Result<()> {
    let p = create_element_attrs!(document, "p",);
    append_element_attrs!(document, p, "canvas", ("id", "chip8-canvas"));
    parent.append_child(&p)?;
    Ok(())
}

fn mount_controls(document: &Document, parent: &HtmlElement) -> Result<()> {
    // containing div
    let div = create_element_attrs!(document, "div", ("id", "chip8canvas"));
    // span
    // TODO pass in state?  5 is hardcoded here, but you havent done state yet.
    //append_text_element_attrs!(
    //    document,
    //    div,
    //    "span",
    //    &format!("{}", STARTING_SIZE),
    //    ("id", "size-output")
    //);
    append_text_element_attrs!(document, div, "label", "Game Loaded:", ("for", "game"));
    let select = create_element_attrs!(document, "select", ("id", "game"));
    for rom in ROMS.keys() {
        append_text_element_attrs!(document, select, "option", rom, ("value", rom));
    }
    div.append_child(&select)?;
    // canvas
    mount_canvas(&document, &div)?;
    parent.append_child(&div)?;
    Ok(())
}

// given a new size, sets all relevant DOM elements
// this is the onChange handler
fn update_all() -> Result<()> {
    //// get new size
    //let document = get_document()?;
    //let new_size = document
    //    .get_element_by_id("size")
    //    .unwrap()
    //    .dyn_into::<web_sys::HtmlInputElement>()?
    //    .value()
    //    .parse::<u32>()
    //    .expect("Could not parse slider value");
    //update_canvas(&document, new_size)?;
    //update_span(&document, new_size)?;
    Ok(())
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
        let body = document.body().unwrap();
        mount_app(&document, &body).unwrap();
        attach_listener(&document).unwrap();
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
    let mut machine = Machine::new(context);
    // TODO get from webpage, which should present a list of options
    machine
        .load_game("test_opcode")
        .expect("Could not load rom");
    if let Err(e) = machine.run() {
        eprintln!("Error: {}", e);
    }
}
