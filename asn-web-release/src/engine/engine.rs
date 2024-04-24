use asn_logger::trace;

use asn_core::traits::TAsnBaseEngine;
use asn_wgpu_released::run_loop;

pub struct Engine {
    is_need_exit: bool,
}

impl TAsnBaseEngine for Engine {
    fn is_need_exit(&self) -> bool {
        self.is_need_exit
    }
    fn set_need_exit(&mut self) {
        self.is_need_exit = true
    }
}

impl Engine {
    pub fn new() -> Self {
        trace!("Engine:new");
        Engine {
            is_need_exit: false,
        }
    }
    pub fn init(&mut self) {
        trace!("Engine:init")
    }
    pub fn run(&mut self) {
        trace!("Engine:run");
        run_loop(self);
    }
}
