use crate::engine::event_runner::EventRunner;

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
    pub fn init(&mut self) {
        println!("Engine:init")
    }
    pub fn run(&mut self) -> ! {
        event_runner::run()
    }
}
