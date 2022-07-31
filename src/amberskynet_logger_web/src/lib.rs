mod utils {
    extern crate wasm_bindgen;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = console)]
        pub fn log(s: &str);
    }
}

pub struct LoggerWeb {}

impl LoggerWeb {
    pub fn log(mess: &str) {
        utils::log(mess);
    }
}
