use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/helpers.js")]
extern "C" {
	fn loadImage(path: &str) -> js_sys::Promise;
	fn sayHello();
}

pub fn say_hello() {
	sayHello();
}
