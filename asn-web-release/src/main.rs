use crate::engine::{init, run};

mod engine;
mod handler;

fn main() {
    let (mut e, mut r) = init();
    run(&mut e, &mut r);
}
