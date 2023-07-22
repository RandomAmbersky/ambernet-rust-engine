mod handler;

use crate::library::engine::handler::Handler;
use asn_core::{AsnContext, AsnEngineTrait};
use asn_runner_winit::{run, WinapiPreset};

pub struct Engine {}

impl Engine {
    pub fn new() -> Self {
        Engine {}
    }
}

impl AsnEngineTrait for Engine {
    fn run(&mut self) {
        let mut preset: WinapiPreset<Handler> = WinapiPreset::new();
        let ctx = AsnContext::new();
        let handler = Handler::new();
        preset.set_ctx(ctx);
        preset.set_handler(handler);
        run(preset);
    }
}
