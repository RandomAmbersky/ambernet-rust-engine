use asn_core::{AsnContext, AsnEvent, AsnHandlerTrait};
use asn_runner_winit::{Runner, RunnerBuilder, WinApi};

mod handler;
pub use handler::get_handler;

pub type Context = AsnContext<WinApi>;

pub fn get_context() -> (Runner, Context) {
    let mut runner_builder = RunnerBuilder::new();
    let winapi = runner_builder.new_winapi();
    let ctx = Context::new(winapi);

    let runner = runner_builder.build();
    (runner, ctx)
}
