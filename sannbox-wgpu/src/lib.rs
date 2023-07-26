use crate::handler::Handler;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

mod engine;
mod handler;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn start() {
    println!("Hello, world!");
    let mut e = engine::Engine::new();
    e.init();

    let h = Handler::new(&mut e);
    e.run(h);
}
