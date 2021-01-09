mod utils;
use utils::log as log_utils;

pub trait Logger {
    fn log(message: &str);
}

pub struct WASMLogger {}
impl Logger for WASMLogger {
    fn log(message: &str) {
        log_utils(message);
    }
}
