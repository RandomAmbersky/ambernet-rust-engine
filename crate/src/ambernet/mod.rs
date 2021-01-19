pub mod api;
pub mod system;
pub mod core;

use std::collections::HashMap;
use uuid::Uuid;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use crate::ambernet::api::{SystemType, SystemBox};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

pub struct AmberNet {
    sys: HashMap<SystemType, SystemBox>
}

impl AmberNet {
    pub fn new() -> Self {
        AmberNet {
            sys: HashMap::new()
        }
    }
    pub fn add_system_box(&mut self, system: SystemBox) -> &mut AmberNet
    {
        let uuid = Uuid::new_v4();
        let mess = format!("add system: {}",  uuid);
        log(&mess);
        self.sys.insert(uuid, system);
        self
    }
    pub fn update(&self, _time: f32) {
        for (_, value) in self.sys.iter() {
            value.process();
        }
    }
}
