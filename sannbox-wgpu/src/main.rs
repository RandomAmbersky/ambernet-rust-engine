use crate::handler::Handler;

mod engine;
mod handler;

fn main() {
    println!("Hello, world!");
    let mut e = engine::Engine::new();
    e.init();

    let h = Handler::new(&mut e);
    e.run(h);
}
