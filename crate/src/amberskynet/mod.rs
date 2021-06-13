pub use engine::Engine as EngineWebGl;

use crate::amberskynet::api::AmberNetApi;

pub mod api;
pub mod logger;
pub mod render;
pub mod engine;

pub fn get_engine () -> EngineWebGl{
    EngineWebGl::new()
}

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
        console_error_panic_hook::set_once();
}
