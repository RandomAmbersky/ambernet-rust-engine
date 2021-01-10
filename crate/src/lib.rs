extern crate wasm_bindgen;
extern crate web_sys;

mod utils;
mod amberskynet;

use wasm_bindgen::prelude::*;

use utils::set_panic_hook;
use amberskynet::{
    app,
    AmberNetApi,
    LoggerApi
};

// Called by our JS entry point to run the example
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    set_panic_hook();

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    val.set_inner_html("Hello from Rust, WebAssembly, and Parcel!");

    body.append_child(&val)?;

    let app = app();
    let engine = app.get_engine();
    let logger = engine.get_log();
    logger.log("AmberSkyNet forever...");

    Ok(())
}
