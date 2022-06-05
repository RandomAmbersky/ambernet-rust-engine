use std::fmt::Debug;
use std::future::Future;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen(module = "/src/helpers.js")]
extern "C" {
	fn loadImage(path: &str) -> js_sys::Promise;
	fn sayHello();
}

pub fn say_hello() {
	sayHello();
}
