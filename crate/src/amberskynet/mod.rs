// use crate::utils::log as utils_log;
// use super::logger::Logger;
// use super::logger::WASMLogger;
// use wasm_bindgen::__rt::std::sync::WaitTimeoutResult;

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

pub trait LoggerApi {
    fn log(&self, mess: &str);
}

pub struct LoggerWebGl {}

impl LoggerApi for LoggerWebGl {
    fn log(&self, mess: &str) {
        webgl_log(mess)
    }
}

pub trait AmberNetApi<LoggerType> {
    fn new() -> Self;
    fn log(&self, mess: &str);
    fn get_log(&self) -> &LoggerType;
}

pub struct AmberNet<LoggerType> {
    logger: LoggerType
}

impl AmberNetApi<LoggerWebGl> for AmberNet<LoggerWebGl> {
    fn new() -> Self {
        Self {
            logger: LoggerWebGl{}
        }
    }

    fn log(&self, mess: &str) {
        &self.logger.log(mess);
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
    pub fn log(&self, mess: &str) {
        &self.engine.logger.log(mess);
    }
}

pub fn get_app() -> AppWebGl {
    AppWebGl::new()
}

// impl AmberNetApi for AmberNetWebGL {
//     fn log(mess: &str) {
//         unimplemented!()
//     }
// }

// trait AmberSkyNetInterface<LoggerType> {
//     fn new(name: String) -> Self;
//     fn update(&self, time: f32);
//     fn render(&self);
//     fn set_logger(logger: &LoggerType);
// }

// pub struct AmberSkyNetStruct<'ambernet, LoggerType> {
//     name: String,
//     logger: &'ambernet LoggerType
// }

// impl AmberSkyNetInterface<LoggerType> for AmberSkyNet<> {
//     fn new(name: String) -> Self {
//         Self {
//             name: _name,
//             logger: &WASMLogger{}
//         }
//     }
//
//     fn update(&self, time: f32) {
//         unimplemented!()
//     }
//
//     fn render(&self) {
//         unimplemented!()
//     }
//
//     fn set_logger(logger: &WASMLogger) {
//         unimplemented!()
//     }
// }

// impl AmberSkyNet {
//     fn new(name: String) -> Self;
//     fn update(&self, time: f32);
//     fn render(&self);
// }

// impl AmberSkyNet for AmberNetEmpty {
//     fn new(_name: String) -> Self {
//         let _str_out = format!("AmberSkyNet new '{}'", &_name);
//         // utils_log(&_str_out);
//         Self {
//             name: _name
//         }
//     }
//
//     fn update(&self, time: f32) {
//         let _str_out = format!("AmberSkyNet update '{}' {}", &self.name, time);
//         // utils_log(&_str_out);
//     }
//
//     fn render(&self) {
//         let _str_out = format!("AmberSkyNet render '{}'", &self.name);
//         // utils_log(&_str_out);
//     }
// }
