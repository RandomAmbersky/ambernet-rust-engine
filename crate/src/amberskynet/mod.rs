// use crate::utils::log as utils_log;

pub trait AmberSkyNet {
    fn new(name: String) -> Self;
    fn update(&self, time: f32);
    fn render(&self);
}

pub struct AmberNetEmpty {
    name: String
}

impl AmberSkyNet for AmberNetEmpty {
    fn new(_name: String) -> Self {
        let _str_out = format!("AmberSkyNet new '{}'", &_name);
        // utils_log(&_str_out);
        Self {
            name: _name
        }
    }

    fn update(&self, time: f32) {
        let _str_out = format!("AmberSkyNet update '{}' {}", &self.name, time);
        // utils_log(&_str_out);
    }

    fn render(&self) {
        let _str_out = format!("AmberSkyNet render '{}'", &self.name);
        // utils_log(&_str_out);
    }
}
