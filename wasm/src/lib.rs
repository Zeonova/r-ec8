/// $ wasm-pack build --target web
/// $ mv pkg/wasm_bg.wasm ../web
/// $ mv pkg/wasm.js ../web
use chip8_core::*;
use console_log::init_with_level;
use js_sys::Uint8Array;
use log::{info, warn, Level};
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, KeyboardEvent};

#[wasm_bindgen]
pub struct EmuWasm {
    chip8: Emu,
    ctx: CanvasRenderingContext2d,
}
#[wasm_bindgen]
impl EmuWasm {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<EmuWasm, JsValue> {
        let _ = init_with_level(Level::Debug);
        console_error_panic_hook::set_once();

        info!("be works!");

        let chip8 = Emu::default();

        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: HtmlCanvasElement = canvas
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        Ok(EmuWasm { chip8, ctx })
    }
}
impl Default for EmuWasm {
    fn default() -> EmuWasm {
        Self::new().unwrap()
    }
}
#[wasm_bindgen]
impl EmuWasm {
    // Wrappers to call corresponding functions in the chip8_core.
    #[wasm_bindgen]
    pub fn tick(&mut self) {
        info!("tick!");
        self.chip8.tick();
    }

    #[wasm_bindgen]
    pub fn tick_timers(&mut self) {
        info!("tick_timers!");
        self.chip8.tick_timers();
    }
    #[wasm_bindgen]
    pub fn reset(&mut self) {
        info!("reset!");

        self.chip8.reset();

        info!("reset done!");
    }
    #[wasm_bindgen]
    pub fn keypress(&mut self, evt: KeyboardEvent, pressed: bool) {
        info!("keypress!");

        let key = evt.key();
        if let Some(k) = key2btn(&key) {
            self.chip8.keypress(k, pressed);
        }
    }
    #[wasm_bindgen]
    pub fn load_game(&mut self, data: Uint8Array) {
        info!("load game!");

        if data.is_null() {
            warn!("Game data is empty!");
        }
        self.chip8.load(&data.to_vec());
    }
    #[wasm_bindgen]
    pub fn draw_screen(&mut self, scale: usize) {
        info!("draw screen!");

        let disp = self.chip8.get_display();
        for i in 0..(SCREEN_H * SCREEN_W) {
            if disp[i] {
                let x = i % SCREEN_W;
                let y = i / SCREEN_W;
                self.ctx.fill_rect(
                    (x * scale) as f64,
                    (y * scale) as f64,
                    scale as f64,
                    scale as f64,
                );
            }
        }
    }
}

fn key2btn(key: &str) -> Option<usize> {
    info!("bey2btn...!");

    match key {
        "1" => Some(0x1),
        "2" => Some(0x2),
        "3" => Some(0x3),
        "4" => Some(0xC),
        "q" => Some(0x4),
        "w" => Some(0x5),
        "e" => Some(0x6),
        "r" => Some(0xD),
        "a" => Some(0x7),
        "s" => Some(0x8),
        "d" => Some(0x9),
        "f" => Some(0xE),
        "z" => Some(0xA),
        "x" => Some(0x0),
        "c" => Some(0xB),
        "v" => Some(0xF),
        _ => None,
    }
}
