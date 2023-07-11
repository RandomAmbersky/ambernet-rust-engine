#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use asn_logger::info;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
// #[allow(dead_code)]
extern "C" {
    #[cfg(target_arch = "wasm32")]
    fn alert(s: &str);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn greet(name: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        alert(&format!("Hello, {}!", name));
    }
    info!("Hello {:}", name);
}
