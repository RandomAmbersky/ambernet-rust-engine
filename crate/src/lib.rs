extern crate wasm_bindgen;
extern crate web_sys;

mod ambernet;
mod utils;

use ambernet::AmberNet;
use wasm_bindgen::prelude::*;
use crate::utils::set_panic_hook;
use crate::ambernet::system as sys;

#[wasm_bindgen]
pub struct AmberSkyNet {
    a: AmberNet
}

#[wasm_bindgen]
impl AmberSkyNet {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        set_panic_hook();
        let mut a = AmberNet::new();
        let render_box = Box::new(sys::get_renderer());
        let sound_box = Box::new(sys::get_sound());
        a.add_system_box(render_box);
        a.add_system_box(sound_box);
        Self { a }
    }
    pub fn update(&self, _time: f32) {
        self.a.update(_time);
    }
    pub fn resize(&self, _width: f32, _height: f32) {}
    pub fn render(&self) {}
}
