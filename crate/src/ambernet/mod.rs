use std::collections::HashMap;
use uuid::Uuid;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

pub type SystemType = uuid::Uuid;
pub type SystemBox = Box<dyn SystemApi>;

pub struct AmberNet {
    sys: HashMap<SystemType, SystemBox>
}

pub trait SystemApi {
    fn get_box(&self) -> SystemBox;
    fn process(&self);
}

pub struct SystemLog {
}

impl SystemApi for SystemLog {
    fn get_box(&self) -> SystemBox {
        let s = Self {};
        Box::new(s)
    }

    fn process(&self) {
        log("SystemLog process!");
    }
}

pub struct SystemRender {
}

impl SystemApi for SystemRender {
    fn get_box(&self) -> SystemBox {
        let r = Self {};
        Box::new(r)
    }

    fn process(&self) {
        log("SystemRender process!");
    }
}

impl AmberNet {
    pub fn new() -> AmberNet {
        let mut a = AmberNet {
            sys: HashMap::new()
        };
        let log = Box::new(SystemLog{});
        let render = Box::new(SystemRender{});
        a.add_system_box(render);
        a.add_system_box(log);
        a
    }
    pub fn add_system_box(&mut self, system: SystemBox)
    {
        let uuid = Uuid::new_v4();
        let mess = format!("add system: {}",  uuid);
        log(&mess);
        self.sys.insert(uuid, system);
    }
    pub fn update(&self, _time: f32) {
        for (key, value) in self.sys.iter() {
            self.sys.get(key).unwrap().process();
        }
    }
}
