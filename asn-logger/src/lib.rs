mod asn_log_level;
mod log_level;
mod mapper;

use crate::mapper::convert;
pub use asn_log_level::AsnLogLevel;
pub use log::{debug, info, log, trace, warn, Level};

pub fn init_log(l: AsnLogLevel) {
    let log_level_filter = convert(l);
    // cfg_if::cfg_if! {
    //     if #[cfg(target_arch = "wasm32")] {
    //         std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    //         let level_option = log_level_filter.to_level();
    //         if level_option.is_some() {
    //             console_log::init_with_level(level_option.unwrap()).expect("Couldn't initialize logger");
    //         }
    //     } else {
    log_level::init_log(log_level_filter);
    log!(target: "app_events", Level::Warn, "App warning");
    // env_logger::init();
    // }
    // }
}
