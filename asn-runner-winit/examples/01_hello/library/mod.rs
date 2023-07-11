use asn_core::AsnContext;
use asn_runner_winit::{Runner, WinApi};

pub mod handler;

pub type Context = AsnContext<WinApi>;

pub fn get_context() -> Context {
    let runner = Runner::new();
    let winapi = runner.new_winapi();
    let ctx = Context::new(winapi);
    ctx
}
