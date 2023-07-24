use crate::engine::TAsnEngine;
use crate::handler::Handler;

mod engine;
mod handler;

fn main() {
    println!("Hello, world!");
    let mut e = engine::Engine::new();
    let handler = handler::Handler::new();
    e.init();
    e.run(handler);
}
