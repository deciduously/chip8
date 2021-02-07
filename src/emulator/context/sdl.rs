use super::*;
use sdl2::{
    self,
    audio::{AudioCallback, AudioSpecDesired},
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::WindowCanvas,
    AudioSubsystem, EventPump,
};
use std::time::Duration;

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
    audio: AudioSubsystem,
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
        let audio = context.audio().unwrap();
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
            audio,
            canvas,
            event_pump,
            key_state: Keys::new(),
        };

        Box::new(ret)
    }
}

// This is literally the example from https://docs.rs/sdl2/0.34.3/sdl2/audio/index.html
// Used for the system beep
struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

impl Context for SdlContext {
    fn init(&mut self) {
        self.canvas.clear();
        self.canvas.present();
    }
    fn beep(&self) {
        let desired_spec = AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1),
            samples: None,
        };
        let device = self
            .audio
            .open_playback(None, &desired_spec, |spec| SquareWave {
                phase_inc: 440.0 / spec.freq as f32,
                phase: 0.0,
                volume: 0.25,
            })
            .unwrap();
        device.resume();
        self.sleep(25);
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
                    if let Some(key) = keycode_to_keypad(keycode) {
                        self.key_state.key_down(key);
                    }
                }
                Event::KeyUp { keycode, .. } => {
                    if let Some(key) = keycode_to_keypad(keycode) {
                        self.key_state.key_up(key);
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
        self.key_state.clone()
    }

    fn random_byte(&self) -> u8 {
        rand::random::<u8>()
    }

    fn sleep(&self, millis: u64) {
        std::thread::sleep(Duration::from_millis(millis));
    }
}
