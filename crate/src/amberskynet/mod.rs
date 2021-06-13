pub use engine::Engine as EngineWebGl;

use crate::amberskynet::api::AmberNetApi;

pub mod api;
pub mod logger;
pub mod render;
pub mod engine;

pub fn get_engine () -> EngineWebGl{
    EngineWebGl::new()
}
