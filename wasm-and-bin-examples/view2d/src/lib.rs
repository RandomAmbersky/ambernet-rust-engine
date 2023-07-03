use wasm_bindgen::prelude::*;

use rs_amberskynet::{AsnContext, ExtHandlerTrait};
use std::fmt::Error;

struct Handler {}

impl Handler {
    fn new(_ctx: &AsnContext) -> Result<Self, Error> {
        let h = Handler {};
        Ok(h)
    }
}

impl ExtHandlerTrait for Handler {
    fn draw(&mut self, _e: &mut AsnContext) {}
    fn update(&mut self, _e: &mut AsnContext) {}
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn start() {
    let (ctx, event_loop) = rs_amberskynet::init();
    if let Ok(_t) = Handler::new(&ctx) {
        rs_amberskynet::run(ctx, event_loop, _t)
    };
}
