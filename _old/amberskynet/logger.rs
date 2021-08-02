mod utils {
    extern crate wasm_bindgen;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = console)]
        pub fn log(s: &str);
    }
}

use crate::amberskynet::api;
use utils::log as webgl_log;

pub struct LoggerWebGl {}

impl LoggerWebGl {
    pub fn log(mess: &str) {
        webgl_log(mess)
    }
}

impl api::LoggerApi for LoggerWebGl {
    fn log(&self, mess: &str) {
        webgl_log(mess)
    }
}
