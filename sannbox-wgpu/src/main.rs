use crate::engine::AsnEngine;

mod engine;

fn main() {
    println!("Hello, world!");
    let mut e = engine::Engine::new();
    e.init();
    e.run();
}
