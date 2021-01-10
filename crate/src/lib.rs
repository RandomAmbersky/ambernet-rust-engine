extern crate wasm_bindgen;
extern crate web_sys;

mod utils;
mod amberskynet;

use wasm_bindgen::prelude::*;

use utils::set_panic_hook;

use amberskynet::{LoggerWebGl, LoggerApi};

// use logger::{
//     Logger,
//     WASMLogger
// };

// use amberskynet::AmberSkyNet;

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

// fn say_hello_from_rust() {
//     WASMLogger::log("Howdy!... from Rust =)");
// }

// Called by our JS entry point to run the example
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    // If the `console_error_panic_hook` feature is enabled this will set a panic hook, otherwise
    // it will do nothing.
    set_panic_hook();

    // say_hello_from_rust();
    // log("AmberSkyNet forever...");

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    val.set_inner_html("Hello from Rust, WebAssembly, and Parcel!");

    body.append_child(&val)?;

    let logger = LoggerWebGl{};
    logger.log("Hello!");

    // let app = AppWebGl::new();

    // let engine = app.get_api();

    // let log:&LoggerWebGl = engine.get_console();

    // WASMLogger::log("lol");

    // let a = AmberSkyNet::set_logger();

    // let a = AmberNetEmpty::new(
    //     String::from("my cool engine")
    // );
    // a.render();
    // a.update(50.03);

    Ok(())
}
