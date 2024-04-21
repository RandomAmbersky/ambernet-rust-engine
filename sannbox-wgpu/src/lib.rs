extern crate core;

use crate::handler::Handler;
use asn_logger::AsnLogLevel;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

mod engine;
mod handler;
mod map;
mod tileset;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct EngineRunner {
    e: engine::Engine,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl EngineRunner {
    pub fn start() {
        println!("Hello, world!");
        asn_logger::init_log(AsnLogLevel::Trace);

        let mut e = engine::Engine::new();
        e.init();

        let h = Handler::new(&mut e);
        e.run(h);
    }
    pub fn resize(&mut self) {
        // self.e.resize();
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn init() -> EngineRunner {
    let mut e = engine::Engine::new();
    let runner = EngineRunner { e };
    runner
}

// #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
// pub fn start() {
//     println!("Hello, world!");
//     asn_logger::init_log(AsnLogLevel::Info);
//
//     let mut e = engine::Engine::new();
//     e.init();
//
//     let h = Handler::new(&mut e);
//     e.run(h);
// }
