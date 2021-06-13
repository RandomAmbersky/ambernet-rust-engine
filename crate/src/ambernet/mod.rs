pub mod system;

use std::collections::HashMap;
use uuid::Uuid;

pub trait SystemApi {
    fn process(&self);
}

pub type UID = uuid::Uuid;
pub type SystemBox = Box<dyn SystemApi>;

pub struct Logger {}

impl Logger {
    fn log(s: &str) {
        log(s);
    }
}

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

pub struct AmberNet {
    sys: HashMap<UID, SystemBox>
}

impl AmberNet {
    pub fn new() -> Self {
        Self {
            sys: HashMap::new()
        }
    }
    pub fn add_system_box(&mut self, system: SystemBox) -> &mut AmberNet
    {
        let uuid = Uuid::new_v4();
        let mess = format!("add system: {}",  uuid);
        Logger::log(&mess);
        self.sys.insert(uuid, system);
        self
    }
    pub fn update(&self, _time: f32) {
        for (_, value) in self.sys.iter() {
            value.process();
        }
    }
}
