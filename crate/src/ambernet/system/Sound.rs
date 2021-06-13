use crate::ambernet::{SystemApi, Logger};

pub struct Sound {}

impl Sound {
    pub fn new() -> Self {
        Self{}
    }
}

impl SystemApi for Sound {
    fn process(&self) {
        let mess = format!("Sound processed...");
        Logger::log(&mess);
    }
}
