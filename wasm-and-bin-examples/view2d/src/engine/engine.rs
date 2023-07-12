use crate::engine::handler::Handler;
use asn_core::AsnContext;
use asn_runner_winit::{Runner, WinApi};

pub struct Engine {
    runner: Option<Runner>,
    ctx: Option<Context>,
    handler: Option<Handler>,
}

pub type Context = AsnContext<WinApi>;

impl Engine {
    pub fn new() -> Self {
        let runner = Runner::new();
        let winapi = runner.new_winapi();
        let ctx = Context::new(winapi);
        let handler = Handler::new();
        Engine {
            runner: Some(runner),
            ctx: Some(ctx),
            handler: Some(handler),
        }
    }
    pub fn run(&mut self) {
        let ctx = self.ctx.take().unwrap();
        let handler = self.handler.take().unwrap();
        let runner = self.runner.take().unwrap();
        runner.run(ctx, handler);
    }
}
