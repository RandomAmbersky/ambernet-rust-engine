use rs_amberskynet;
use rs_amberskynet::{AsnContext, ExtHandlerTrait};
use std::fmt::Error;

struct Handler {}

impl Handler {
    fn new(_ctx: &AsnContext) -> Result<Self, Error> {
        let h = Handler {};
        Ok(h)
    }
}

impl ExtHandlerTrait for Handler {
    fn draw(&mut self, _e: &mut AsnContext) {}
    fn update(&mut self, _e: &mut AsnContext) {}
}

pub fn start() {
    let (ctx, event_loop) = rs_amberskynet::init();
    if let Ok(_t) = Handler::new(&ctx) {
        rs_amberskynet::run(ctx, event_loop, _t)
    };
}
