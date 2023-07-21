mod engine;

use engine::Engine;

pub fn get_engine() -> Engine {
    Engine::new()
}
