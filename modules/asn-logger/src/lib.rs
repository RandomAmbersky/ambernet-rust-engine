extern crate log;

mod asn_log_level;
mod log_level;
mod mapper;

use crate::mapper::convert;
pub use asn_log_level::AsnLogLevel;
use log::{debug, error, info, trace, warn};

pub fn init_log(l: AsnLogLevel) {
    let log_level_filter = convert(l);
    log_level::init_log(log_level_filter);

    cfg_if::cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            #[cfg(feature = "test_messages")]
            debug("asn-logger", "console_error_panic_hook enabled");
        }
    }

    #[cfg(feature = "test_messages")]
    {
        trace("asn-logger", "App tracing");
        warn("asn-logger", "App warning");
        info("asn-logger", "App info");
        debug("asn-logger", "App debug");
        error("asn-logger", "App error");
    }
}

pub fn trace(t: &str, s: &str) {
    trace!(target: t, "{}", s);
}

pub fn warn(t: &str, s: &str) {
    warn!(target: t, "{}", s);
}

pub fn info(t: &str, s: &str) {
    info!(target: t, "{}", s);
}

pub fn debug(t: &str, s: &str) {
    debug!(target: t, "{}", s);
}

pub fn error(t: &str, s: &str) {
    error!(target: t, "{}", s);
}
