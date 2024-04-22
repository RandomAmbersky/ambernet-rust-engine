use asn_logger::trace;

pub struct Engine {
    is_need_exit: bool,
}

impl Engine {
    pub fn new() -> Self {
        trace!("Engine:new");
        Engine {
            is_need_exit: false,
        }
    }
    pub fn init(&mut self) {
        // println!("Engine:init")
    }
    pub fn run(mut self) {}
}
