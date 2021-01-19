use crate::ambernet::api::{SystemApi};

pub struct SystemLog {
}

impl SystemLog {
    pub(crate) fn new() -> Self {
        SystemLog{}
    }
}

impl SystemApi for SystemLog {
    fn process(&self) {
        log("SystemLog process!");
    }
}

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}
