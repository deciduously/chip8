//! This module builds the containing webpage and mounts the machine to a canvas element.
use std::{cell::RefCell, rc::Rc, sync::{Arc, RwLock}};

use super::*;
use crate::ROMS;
use console_error_panic_hook::set_once;
use js_sys::{Math::{floor, random}};
use lazy_static::lazy_static;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, Document, Element, HtmlElement, Window};

type Result<T> = std::result::Result<T, JsValue>;

const INSTRUCTIONS: &str = "Select your preferred ROM, and use the keys as shown.  Use the 'G' key to restart the current ROM.

  CHIP8   =>  Keyboard

|1|2|3|C| => |1|2|3|4|
|4|5|6|D| => |Q|W|E|R|
|7|8|9|E| => |A|S|D|F|
|A|0|B|F| => |Z|X|C|V|";

// Module-specific static storage for key state and game selection

lazy_static! {
    static ref KEYS: Keys = Keys::new(); // TODO I think the Arc should jus tbe here, dont make Machine worry about it
    static ref CURRENT_GAME: Arc<RwLock<String>> = Arc::new(RwLock::new("test_opcode".to_string()));
    static ref TRIGGER_RESTART: Arc<RwLock<bool>> = Arc::new(RwLock::new(false));
}

// Logging/error convenience macros for println!-=seque usage

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

/*
macro_rules! error {
    ( $( $t:tt )* ) => {
        web_sys::console::error_1(&format!( $( $t )* ).into());
    }
}
*/

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
        } else if c == 'G' {
            // trigger restart
            *TRIGGER_RESTART.write().unwrap() = true;
        }
    }) as Box<dyn FnMut(_)>);

    document.add_event_listener_with_callback("keydown", callback.as_ref().unchecked_ref())?;

    callback.forget();
    Ok(())
}

/// Keyup event listener
fn attach_keyup_listener(document: &Document) -> Result<()> {
    let callback = Closure::wrap(Box::new(move |evt: web_sys::Event| {
        let evt = evt.dyn_into::<web_sys::KeyboardEvent>().unwrap();
        let c = std::char::from_u32(evt.key_code()).unwrap();
        if let Ok(ch) = keyboard_to_keypad(c) {
            KEYS.key_up(ch);
        }
    }) as Box<dyn FnMut(_)>);

    document.add_event_listener_with_callback("keyup", callback.as_ref().unchecked_ref())?;

    callback.forget();
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
        let selected = rom == &*CURRENT_GAME.read().unwrap();
        let new_option =
            web_sys::HtmlOptionElement::new_with_text_and_value_and_default_selected_and_selected(
                rom, rom, selected, selected,
            )?;
        select.append_child(&new_option)?;
    }
    div.append_child(&select)?;
    // canvas
    mount_canvas(&document, &div)?;
    parent.append_child(&div)?;
    Ok(())
}

/// Used to defocus all elements when a new game is chosen.
/// Otherwise player input will be received as a new selection
fn blur_all() -> Result<()> {
    let document = get_document();
    let tmp = create_element_attrs!(document, "input",);
    let body = document.body().expect("Should find body");
    let tmp = tmp.dyn_into::<HtmlElement>()?;

    body.append_child(&tmp)?;
    tmp.focus()?;
    body.remove_child(&tmp)?;
    Ok(())
}

// Whena new game is selected, pass it to the machine
// this is the onChange handler
fn update_all() -> Result<()> {
    // get new game
    let document = get_document();
    let new_game_select = document
        .get_element_by_id("game")
        .unwrap()
        .dyn_into::<web_sys::HtmlSelectElement>()?;
    // Load new game
    *CURRENT_GAME.write().unwrap() = new_game_select.value().to_string();
    Ok(())
}

// draw screen
fn update_canvas(
    context: &CanvasRenderingContext2d,
    width: f64,
    height: f64,
    screen: Screen,
) -> Result<()> {
    // draw

    context.set_fill_style(&JsValue::from_str("black"));
    context.fill_rect(0.0, 0.0, width, height);

    // For pixel in screen
    context.set_fill_style(&JsValue::from_str("white"));
    for y in 0..PIXEL_ROWS {
        for x in 0..PIXEL_COLS {
            // Draw a point if it exists
            if screen[(x + (y * PIXEL_COLS)) as usize] == 1 {
                context.fill_rect(x as f64, y as f64, 1.0, 1.0);
            }
        }
    }
    Ok(())
}

fn window() -> Window {
    web_sys::window().expect("no global `window` exists")
}

fn get_document() -> Document {
    window()
        .document()
        .expect("Should have a document on the window")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

/// Render a string for the console
#[allow(dead_code)]
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
    ctx: Option<CanvasRenderingContext2d>,
    width: u32,
    height: u32,
    scale_factor: u32,
}

impl WasmContext {
    pub fn new(scale_factor: u32) -> Box<Self> {
        Box::new(Self {
            ctx: None,
            width: PIXEL_COLS * scale_factor,
            height: PIXEL_ROWS * scale_factor,
            scale_factor,
        })
    }
}

impl Context for WasmContext {
    fn init(&mut self) {
        // First, mount the DOM
        mount();
        let document = get_document();

        // grab canvas
        let canvas = document
            .get_element_by_id("chip8-canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();
        canvas.set_width(self.width);
        canvas.set_height(self.height);
        // TODO pass attribute to disable alpha - performance?
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        context
            .scale(self.scale_factor as f64, self.scale_factor as f64)
            .unwrap();
        self.ctx = Some(context);
        log!("Finished init");
    }
    fn beep(&self) {}
    fn listen_for_input(&mut self) -> bool {
        // TODO listen for quit??
        false
    }
    fn draw_graphics(&mut self, screen: Screen) {
        //debug_render(screen);
        update_canvas(
            self.ctx.as_ref().unwrap(),
            self.width as f64,
            self.height as f64,
            screen,
        )
        .unwrap();
    }
    fn get_key_state(&self) -> [bool; NUM_KEYS] {
        KEYS.inner()
    }
    fn random_byte(&self) -> u8 {
        floor(random() * floor(255.0)) as u8
    }
    fn sleep(&self, millis: u64) {
        // unused?
        let start = js_sys::Date::now();
        let mut current = start;
        while current - start < millis as f64 {
            current = js_sys::Date::now();
        }
    }
}

/// Mount the DOM necessary to host the app
fn mount() {
    set_once(); // console_error_panic_hook
    let document = get_document();
    let body = document.body().unwrap();
    mount_app(&document, &body).unwrap();
    attach_game_listener(&document).unwrap();
    attach_keydown_listener(&document).unwrap();
    attach_keyup_listener(&document).unwrap();
}

#[wasm_bindgen]
pub fn run() {
    // Init context and machine
    let context = WasmContext::new(15);
    let mut machine = Machine::new(context);
    let default_game = &*CURRENT_GAME.read().unwrap();
    let bytes = machine.load_game(default_game).unwrap();
    log!("Loaded {}: {} bytes.", default_game, bytes);

    // see https://rustwasm.github.io/wasm-bindgen/examples/request-animation-frame.html
    // We need to use Rc to store the callback.  One copy will store the callback and kick it off,
    // referencing the same callback through a different pointer

    // Set up the Rc.  g is just a reference to f.
    let f = Rc::new(RefCell::new(None));
    let g = Rc::clone(&f);

    // Counters to force redraw after a certian number of identical frames, even if draw_flag isnt set
    // Otherwise, completed programs will simply freeze the page.
    let max_timeout = 20;
    let mut current_timeout = 0;

    // Store the callback in g (and consequently, f)
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        // First, check if we need to load a new game
        let selected_game = &*CURRENT_GAME.read().unwrap();
        if let Some(name) = &machine.current_game {
            // If the name in the global doesn't match the machine's loaded game OR there's a restart request
            if name != selected_game || *TRIGGER_RESTART.read().unwrap() {
                *TRIGGER_RESTART.write().unwrap() = false;
                let bytes = machine
                    .load_game(&selected_game)
                    .expect("Could not load new rom");
                log!("Loaded {}: {} bytes.", selected_game, bytes);
                    // Defocus everything to clear way for keybaord input
                    blur_all().unwrap();
            }
        }

        //log!("{}", KEYS.to_string());

        // Then, execute cycles until draw_flag gets set or a certain number of cycles have passed.
        // This is basically machine::step() but in wasm callback form

        loop {
            // TODO listen for a quit signal or something?

            machine.cycle().unwrap();

            machine.update_keys();

            if machine.draw_flag {
                machine.draw_graphics();
                current_timeout = 0;
                break;
            }
            if current_timeout >= max_timeout {
                current_timeout = 0;
                break;
            }
            current_timeout += 1;
        }
        // Schedule another redraw
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    // Kick off initial redraw
    request_animation_frame(g.borrow().as_ref().unwrap());
}
