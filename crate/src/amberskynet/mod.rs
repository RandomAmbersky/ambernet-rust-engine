pub mod api;
mod webgl;

use webgl::engine::Engine as Engine;
use api::{
    AmberNetApi,
};

pub fn get_engine () -> Engine{
    Engine::new()
}
