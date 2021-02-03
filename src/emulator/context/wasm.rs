//! This module builds the containing webpage and mounts the machine to a canvas element.
use super::*;
use crate::ROMS;
use console_error_panic_hook::set_once;
use js_sys::Math::{floor, random};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console::error_2, Document, Element, HtmlElement};

type Result<T> = std::result::Result<T, JsValue>;

const INSTRUCTIONS: &str = "Select your preferred game, and use the keys as shown:

  CHIP8   =>  Keyboard

|1|2|3|C| => |1|2|3|4|
|4|5|6|D| => |Q|W|E|R|
|7|8|9|E| => |A|S|D|F|
|A|0|B|F| => |Z|X|C|V|";

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

macro_rules! error {
    ( $( $t:tt )* ) => {
        web_sys::console::error_1(&format!( $( $t )* ).into());
    }
}

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
        .get_element_by_id("game")
        .unwrap()
        .dyn_into::<web_sys::HtmlSelectElement>()?
        .set_onchange(Some(callback.as_ref().unchecked_ref()));

    callback.forget(); // leaks memory!

    Ok(())
}

fn mount_app(document: &Document, body: &HtmlElement) -> Result<()> {
    append_text_element_attrs!(document, body, "h1", "CHIP-8",);
    mount_controls(&document, &body)?;
    append_text_element_attrs!(document, body, "pre", INSTRUCTIONS,);
    append_text_element_attrs!(
        document,
        body,
        "a",
        "source",
        ("href", "https://github.com/deciduously/chip8")
    );
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

// Whena new game is selected, pass it to the machine
// this is the onChange handler
fn update_all() -> Result<()> {
    //// get new game
    let document = get_document()?;
    let new_game = document
        .get_element_by_id("game")
        .unwrap()
        .dyn_into::<web_sys::HtmlSelectElement>()?
        .value();
    // TODO - tear down machine and build a new one, or load game?
    Ok(())
}

// draw dot
fn update_canvas(document: &Document, screen: Screen) -> Result<()> {
    // grab canvas
    let canvas = document
        .get_element_by_id("chip8-canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    // resize canvas to size * 2
    // TODO slider for scale facotr, a la wasm-dot?
    let scale_factor = 10.0;
    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    // draw

    context.set_fill_style(&JsValue::from_str("black"));
    context.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());

    // For pixel in screen
    context.scale(scale_factor, scale_factor)?;
    context.set_fill_style(&JsValue::from_str("white"));
    for y in 0..PIXEL_ROWS {
        for x in 0..PIXEL_COLS {
            // Draw a point if it exists scaled up from the source screen
            if screen[(x + (y * PIXEL_COLS)) as usize] == 1 {
                context.fill_rect(x as f64, y as f64, 1.0, 1.0);
            }
        }
    }
    Ok(())
}

fn get_document() -> Result<Document> {
    let window = web_sys::window().unwrap();
    Ok(window.document().unwrap())
}

/// Render a string for the console
fn debug_render(screen: Screen) {
    let mut ret = String::new();
    for y in 0..PIXEL_ROWS {
        for x in 0..PIXEL_COLS {
            if screen[((y * PIXEL_COLS) + x) as usize] == 0 {
                ret.push('0');
            } else {
                ret.push(' ');
            }
        }
        ret.push('\n');
    }
    ret.push('\n');
    log!("{}", ret);
}

/// The WebAssembly interface
#[derive(Debug)]
pub struct WasmContext {
    key_state: Keys,
}

impl WasmContext {
    pub fn new() -> Box<Self> {
        let document = get_document().unwrap();
        let body = document.body().unwrap();
        mount_app(&document, &body).unwrap();
        attach_listener(&document).unwrap();
        Box::new(Self {
            key_state: FRESH_KEYS,
        })
    }
}

impl Context for WasmContext {
    fn init(&mut self) {
        // grab canvas
        let canvas = get_document()
            .unwrap()
            .get_element_by_id("chip8-canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();
        // TODO slider for scale factor, a la wasm-dot?
        let scale_factor = 10;
        canvas.set_width(PIXEL_COLS * scale_factor);
        canvas.set_height(PIXEL_ROWS * scale_factor);
    }
    fn beep(&self) {}
    fn listen_for_input(&mut self) -> bool {
        false
    }
    fn draw_graphics(&mut self, screen: Screen) {
        debug_render(screen);
        update_canvas(&get_document().unwrap(), screen).unwrap();
    }
    fn get_key_state(&self) -> Keys {
        self.key_state
    }
    fn random_byte(&self) -> u8 {
        floor(random() * floor(255.0)) as u8
    }
}

#[wasm_bindgen]
pub fn run() {
    // init conosle_error_panic_hook
    set_once();
    // get document, get body, init machine, etc
    // this is the WASM "main"
    // TODO we need to store page sate somewhere.  GOtta be able to talk to the machine better.
    let context = WasmContext::new();
    let mut machine = Machine::new(context);
    // TODO get from webpage, which should present a list of options
    machine
        .load_game("test_opcode")
        .expect("Could not load rom");
    log!("Loaded test_opcode");
    if let Err(e) = machine.run() {
        error!("Error: {}", e);
    }
}
