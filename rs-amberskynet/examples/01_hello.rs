use rs_amberskynet::{context, ExtHandlerTrait};
use std::arch::asm;
use winit::event_loop;
use winit::event_loop::EventLoop;

struct Handler {}
impl Handler {
    pub fn new() -> Self {
        Self {}
    }
}
impl ExtHandlerTrait for Handler {
    fn draw(&self, e: &rs_amberskynet::AsnContext) {
        // todo!()
    }

    fn update(&self, e: &rs_amberskynet::AsnContext) {
        // todo!()
    }
}

pub fn main() {
    let h = Handler::new();
    let (ctx, event_loop) = rs_amberskynet::init();
    rs_amberskynet::run(ctx, event_loop, h)
}
