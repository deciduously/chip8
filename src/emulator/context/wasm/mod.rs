//! This module builds the containing webpage and mounts the machine to a canvas element.
use std::{cell::RefCell, rc::Rc, sync::{Arc, RwLock}};

use super::*;
use crate::ROMS;

use lazy_static::lazy_static;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlElement};

mod dom;
#[macro_use]
mod macros;
mod wasm_context;

use dom::*;
use wasm_context::WasmContext;

pub type Result<T> = std::result::Result<T, JsValue>;

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

fn mount_app(document: &Document, body: &HtmlElement) -> Result<()> {
    append_text_element_attrs!(document, body, "h1", "CHIP-8",);
    mount_controls(&document, &body)?;
    append_text_element_attrs!(
        document,
        body,
        "a",
        "source",
        ("href", "https://github.com/deciduously/chip8"),
        ("target", "_blank")
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
    append_text_element_attrs!(document, parent, "label", "Game Loaded:", ("for", "game"));
    let select = create_element_attrs!(document, "select", ("id", "game"));
    for rom in ROMS.keys() {
        let selected = rom == &*CURRENT_GAME.read().unwrap();
        let new_option =
            web_sys::HtmlOptionElement::new_with_text_and_value_and_default_selected_and_selected(
                rom, rom, selected, selected,
            )?;
        select.append_child(&new_option)?;
    }
    parent.append_child(&select)?;
    
    let div = create_element_attrs!(document, "div", ("id", "chip8canvas"));
    // canvas
    mount_canvas(&document, &div)?;
    parent.append_child(&div)?;
    append_text_element_attrs!(document, parent, "pre", INSTRUCTIONS,);
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

    // Counter to force sleep sometimes
    let mut cycle_counter = 0;

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
                    // Defocus everything to clear way for keyboard input
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

            cycle_counter += 1;
            if cycle_counter >= CYCLES_PER_SLEEP {
                cycle_counter = 0;
                machine.sleep(MILLIS_PER_SLEEP.floor() as u64);
            }
        }
        // Schedule another redraw
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    // Kick off initial redraw
    request_animation_frame(g.borrow().as_ref().unwrap());
}
