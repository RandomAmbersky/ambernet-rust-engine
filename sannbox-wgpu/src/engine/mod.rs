pub struct Engine {}

impl Engine {
    pub fn new() -> Self {
        Engine {}
    }
    pub fn init(&mut self) {
        println!("Engine:init")
    }
    pub fn run(&mut self) {
        println!("Engine:run")
    }
}
