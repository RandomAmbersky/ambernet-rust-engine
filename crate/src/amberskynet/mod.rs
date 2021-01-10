pub mod api;

mod webgl_utils {
    extern crate wasm_bindgen;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = console)]
        pub fn log(s: &str);
    }
}

use webgl_utils::log as webgl_log;

pub struct LoggerWebGl {}

impl api::LoggerApi for LoggerWebGl {
    fn log(&self, mess: &str) {
        webgl_log(mess)
    }
}

pub struct AmberNet<LoggerType> {
    logger: LoggerType
}

impl api::AmberNetApi<LoggerWebGl> for AmberNet<LoggerWebGl> {
    fn new() -> Self {
        Self {
            logger: LoggerWebGl{}
        }
    }

    fn get_log(&self) -> &LoggerWebGl {
        &self.logger
    }
}


pub struct AppWebGl {
    engine: AmberNet<LoggerWebGl>
}

impl AppWebGl {
    pub fn new() -> Self {
        Self {
            engine: AmberNet::new()
        }
    }
    pub fn get_engine(&self) -> &AmberNet<LoggerWebGl> {
        &self.engine
    }
}

pub fn app() -> AppWebGl {
    AppWebGl::new()
}
