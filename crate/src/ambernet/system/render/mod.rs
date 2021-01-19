use crate::ambernet::api::{SystemApi};

pub struct SystemRender {
}

impl SystemRender {
    pub(crate) fn new() -> Self {
        SystemRender{}
    }
}

impl SystemApi for SystemRender {
    fn process(&self) {
        log("SystemRender process!");
    }
}

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}
