use crate::engine::handler::Handler;
use asn_core::{AsnContext, AsnEvent, AsnHandlerTrait};
use asn_runner_winit::{Runner, WinApi};
use std::borrow::Borrow;

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
        let runner = &self.runner;
        // runner.run(&mut self.ctx, &mut self.handler);
        // self.handler.proceed(&mut self.ctx, &AsnEvent::Empty);
    }
}
