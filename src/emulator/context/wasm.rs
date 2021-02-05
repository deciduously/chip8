//! This module builds the containing webpage and mounts the machine to a canvas element.
use std::unimplemented;

use super::*;
use crate::ROMS;
use console_error_panic_hook::set_once;
use js_sys::Math::{floor, random};
use lazy_static::lazy_static;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlElement};

type Result<T> = std::result::Result<T, JsValue>;

const INSTRUCTIONS: &str = "Select your preferred game, and use the keys as shown:

  CHIP8   =>  Keyboard

|1|2|3|C| => |1|2|3|4|
|4|5|6|D| => |Q|W|E|R|
|7|8|9|E| => |A|S|D|F|
|A|0|B|F| => |Z|X|C|V|";

// Module-specific static storage for key state

lazy_static! {
    static ref KEYS: Keys = Keys::new();
}

// Logging/error convenience macros for println!-=seque usage

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

/// Listen for game change events
fn attach_game_listener(document: &Document) -> Result<()> {
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

/// Keydown event listener
fn attach_keydown_listener(document: &Document) -> Result<()> {
    let callback = Closure::wrap(Box::new(move |evt: web_sys::Event| {
        let evt = evt.dyn_into::<web_sys::KeyboardEvent>().unwrap();
        let c = std::char::from_u32(evt.key_code()).unwrap();
        if let Ok(ch) = keyboard_to_keypad(c) {
            KEYS.key_down(ch);
        }
    }) as Box<dyn FnMut(_)>);

    document.add_event_listener_with_callback("keydown", callback.as_ref().unchecked_ref())?;

    callback.forget();
    Ok(())
}

// TODO keyup!

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

// draw screen
fn update_canvas(document: &Document, screen: Screen) -> Result<()> {
    // grab canvas
    let canvas = document
        .get_element_by_id("chip8-canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    // draw

    let w = canvas.width().into();
    let h = canvas.height().into();

    context.clear_rect(0.0, 0.0, w, h);
    context.set_fill_style(&JsValue::from_str("black"));
    context.fill_rect(0.0, 0.0, w, h);

    context.begin_path();
    // For pixel in screen
    // TODO this would be faster as imgdata
    // https://rustwasm.github.io/wasm-bindgen/examples/julia.html
    context.set_fill_style(&JsValue::from_str("white"));
    for y in 0..PIXEL_ROWS {
        for x in 0..PIXEL_COLS {
            // Draw a point if it exists scaled up from the source screen
            if screen[(x + (y * PIXEL_COLS)) as usize] == 1 {
                context.fill_rect(x as f64, y as f64, 1.0, 1.0);
            }
        }
    }
    context.stroke();
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
pub struct WasmContext;

impl WasmContext {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
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
        let scale_factor = 15;
        canvas.set_width(PIXEL_COLS * scale_factor);
        canvas.set_height(PIXEL_ROWS * scale_factor);
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        context
            .scale(scale_factor as f64, scale_factor as f64)
            .unwrap();
        log!("Finished init");
    }
    fn beep(&self) {}
    fn listen_for_input(&mut self) -> bool {
        // TODO listen for quit??
        false
    }
    fn draw_graphics(&mut self, screen: Screen) {
        //debug_render(screen);
        update_canvas(&get_document().unwrap(), screen).unwrap();
    }
    fn get_key_state(&self) -> Keys {
        KEYS.clone()
    }
    fn random_byte(&self) -> u8 {
        floor(random() * floor(255.0)) as u8
    }
    fn sleep(&self, _millis: u64) {
        // Unused for WASM, we use setInterval instead
        unimplemented!()
    }
}

/// Mount the DOM necessary to host the app
fn mount() {
    set_once(); // console_error_panic_hook
    let document = get_document().unwrap();
    let body = document.body().unwrap();
    mount_app(&document, &body).unwrap();
    attach_game_listener(&document).unwrap();
    attach_keydown_listener(&document).unwrap();
}

#[wasm_bindgen]
pub fn run() {
    mount();

    // TODO we need to store page sate somewhere.  Gotta be able to talk to the machine better.
    let context = WasmContext::new();

    let mut machine = Machine::new(context);

    // TODO get from DOM select element
    machine
        .load_game("vbrix")
        .expect("Could not load rom");

    let callback = Closure::wrap(Box::new(move || {
        if let Err(e) = machine.step() {
            error!("Error: {}", e);
        }
    }) as Box<dyn FnMut()>);
    web_sys::window()
        .unwrap()
        .set_interval_with_callback_and_timeout_and_arguments_0(
            callback.as_ref().unchecked_ref(),
            4, // DOM_MIN_TIMEOUT_VALUE
        )
        .unwrap();
    callback.forget();
}
