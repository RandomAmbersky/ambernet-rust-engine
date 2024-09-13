use crate::engine::{init, Engine};

mod engine;
mod handler;

fn main() {
    let mut e = init();
    e.run()
}
