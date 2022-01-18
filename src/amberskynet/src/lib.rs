extern crate wasm_bindgen;
extern crate web_sys;

use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
pub struct AmberSkyNetClient {

}

fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
        console_error_panic_hook::set_once();
}

impl Default for AmberSkyNetClient {
    fn default() -> Self {
        todo!()
    }
}

#[wasm_bindgen]
impl AmberSkyNetClient {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        set_panic_hook();
        Self {}
    }
    pub fn update(&self, _time: f32) -> Result<(), JsValue>{
        // let _mess = format!("engine update: {}", _time);
        // self.a.get_log().log(&mess);
        Ok(())
    }
    pub fn resize(&mut self, _width: f32, _height: f32) -> Result<(), JsValue> {
        // self.a.render().resize(_width, _height);
        Ok(())
    }
    pub fn render(&mut self) -> Result<(), JsValue> {
        // &self.a.render().draw();
        Ok(())
    }
}
