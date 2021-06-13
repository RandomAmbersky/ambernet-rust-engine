use crate::ambernet::{SystemApi, Logger};

pub struct Renderer {}

impl Renderer {
    pub fn new() -> Self {
        Self{}
    }
}

impl SystemApi for Renderer {
    fn process(&self) {
        let mess = format!("Renderer processed...");
        Logger::log(&mess);
    }
}
