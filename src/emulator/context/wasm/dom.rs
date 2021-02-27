//! This module interacts with the DOM to build the page and set up the context.

use super::*;
use console_error_panic_hook::set_once;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, Document, Element, Window};

/// Fake a "sleep" function in a hacky dumb way using the Date object.
pub fn sleep(millis: u64) {
    let start = js_sys::Date::now();
    let mut current = start;
    while current - start < millis as f64 {
        current = js_sys::Date::now();
    }
}

/// beep background
pub fn beep() -> Result<()> {
    let document = get_document();
    let div = document
        .body()
        .unwrap()
        .dyn_into::<Element>()?;
    let class_list = div.class_list();
    class_list.add_1("beep")?;

    let remove = Closure::wrap(Box::new(move || {
        class_list.remove_1("beep").unwrap();
    }) as Box<dyn Fn()>);
    window().set_timeout_with_callback_and_timeout_and_arguments_0(
        remove.as_ref().unchecked_ref(),
        300,
    )?;
    remove.forget();

    Ok(())
}

/// Helper to grab the document object
pub fn get_document() -> Document {
    window()
        .document()
        .expect("Should have a document on the window")
}

/// Wrapper for requestAnimationFrame call
pub fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

/// draw screen
pub fn update_canvas(
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

// When a new game is selected, pass it to the machine
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

fn window() -> Window {
    web_sys::window().expect("no global `window` exists")
}

/// Mount the DOM necessary to host the app
pub fn mount() {
    set_once(); // console_error_panic_hook
    let document = get_document();
    let body = document.body().unwrap();
    mount_app(&document, &body).unwrap();
    attach_game_listener(&document).unwrap();
    attach_keydown_listener(&document).unwrap();
    attach_keyup_listener(&document).unwrap();
}
