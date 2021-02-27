//! This struct connects the machine to the wasm context.

use js_sys::{Math::{floor, random}};
use web_sys::CanvasRenderingContext2d;
use super::*;
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