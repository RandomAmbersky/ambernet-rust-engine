mod utils {
    extern crate wasm_bindgen;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = console)]
        pub fn log(s: &str);
    }
}

use super::super::api;
use utils::log as webgl_log;

pub struct Logger {}

impl api::LoggerApi for Logger {
    fn log(&self, mess: &str) {
        webgl_log(mess)
    }
}
