use crate::engine::handler::Handler;
use asn_core::AsnContext;
use asn_runner_winit::{Runner, WinApi};

pub struct Engine {
    runner: Runner,
    ctx: Context,
    handler: Handler,
}

pub type Context = AsnContext<WinApi>;

impl Engine {
    pub fn new() -> Self {
        let runner = Runner::new();
        let winapi = runner.new_winapi();
        let ctx = Context::new(winapi);
        let handler = Handler::new();
        Engine {
            runner,
            ctx,
            handler,
        }
    }
    pub fn run(&mut self) {
        self.runner.run(&self.ctx, &self.handler);
    }
}
