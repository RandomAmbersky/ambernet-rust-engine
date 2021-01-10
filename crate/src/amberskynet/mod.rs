pub mod api;
mod webgl;

use webgl::engine::Engine as EngineWebGl;
use api::{
    AmberNetApi,
};

pub fn get_engine () -> EngineWebGl{
    EngineWebGl::new()
}
