use asn_core::AsnContext;
use asn_runner_winit::{Runner, WinApi};

mod handler;
pub use handler::get_handler;

pub type Context = AsnContext<WinApi>;

pub fn get_context() -> (Runner, Context) {
    let runner = Runner::new();
    let winapi = runner.new_winapi();
    let ctx = Context::new(winapi);
    (runner, ctx)
}
