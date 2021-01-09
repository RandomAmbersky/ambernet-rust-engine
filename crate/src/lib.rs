mod utils;
mod amberskynet;

extern crate wasm_bindgen;
extern crate web_sys;

use wasm_bindgen::prelude::*;

use crate::utils::*;

fn say_hello_from_rust() {
    log("Howdy!... from Rust =)")
}

#[wasm_bindgen]
pub fn draw() {
    log("draw");
}

// Called by our JS entry point to run the example
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    // If the `console_error_panic_hook` feature is enabled this will set a panic hook, otherwise
    // it will do nothing.
    set_panic_hook();

    say_hello_from_rust();
    log("AmberSkyNet forever...");

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    val.set_inner_html("Hello from Rust, WebAssembly, and Parcel!");

    body.append_child(&val)?;

    Ok(())
}
