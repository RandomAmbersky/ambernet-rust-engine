use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct AsnWebPool {}

#[wasm_bindgen]
impl AsnWebPool {
    pub fn run(&mut self) {}
}

#[wasm_bindgen]
pub fn new_asn_web_pool() -> AsnWebPool {
    AsnWebPool {}
}
