use crate::engine::TAsnEngine;

mod engine;

fn main() {
    println!("Hello, world!");
    let mut e = engine::Engine::new();
    e.init();
    e.run();
}
