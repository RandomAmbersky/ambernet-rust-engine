extern crate wasm_bindgen;
extern crate web_sys;

mod logger;
mod utils;
mod amberskynet;

use wasm_bindgen::prelude::*;

use logger::{
    Logger,
    WASMLogger
};
use utils::set_panic_hook;
use crate::amberskynet::{
    AmberNetEmpty, AmberSkyNet};


// struct AmberNet {}
//
// impl amberskynet::AmberSkyNet for AmberNet {
//     fn new() -> Self {
//         log("AmberSkyNet new");
//         Self {}
//     }
//
//     fn update(&self, time: f32) {
//         log("AmberSkyNet update")
//     }
//
//     fn render(&self) {
//         log("AmberSkyNet render")
//     }
// }

fn say_hello_from_rust() {
    WASMLogger::log("Howdy!... from Rust =)");
}

// Called by our JS entry point to run the example
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    // If the `console_error_panic_hook` feature is enabled this will set a panic hook, otherwise
    // it will do nothing.
    set_panic_hook();

    say_hello_from_rust();
    // log("AmberSkyNet forever...");

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    val.set_inner_html("Hello from Rust, WebAssembly, and Parcel!");

    body.append_child(&val)?;

    WASMLogger::log("lol");

    let a = AmberNetEmpty::new(
        String::from("my cool engine")
    );
    a.render();
    a.update(50.03);

    Ok(())
}
