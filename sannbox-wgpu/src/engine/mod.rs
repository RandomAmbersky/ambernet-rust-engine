use crate::engine::event_runner::EventRunner;

pub use crate::engine::core::traits::AsnEngine;

mod core;
mod event_runner;

pub struct Engine {
    event_runner: EventRunner,
}

impl Engine {
    pub fn new() -> Self {
        let event_runner = EventRunner::new();
        Engine { event_runner }
    }
}

impl AsnEngine for Engine {
    fn init(&mut self) {
        println!("Engine:init")
    }
    fn run(self) {
        let e = self.event_runner;
        event_runner::run(e);
    }
}
