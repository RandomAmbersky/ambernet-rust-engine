use crate::ambernet::{SystemApi, Logger};

pub struct SystemRenderer {}

impl SystemRenderer {
    pub fn new() -> Self {
        Self{}
    }
}

impl SystemApi for SystemRenderer {
    fn process(&self) {
        let mess = format!("SystemRenderer processed...");
        Logger::log(&mess);
    }
}
