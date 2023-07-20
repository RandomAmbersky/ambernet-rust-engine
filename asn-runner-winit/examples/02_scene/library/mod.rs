use asn_core::{AsnContext, AsnEvent, AsnHandlerTrait};
use asn_runner_winit::{AsnScene, Runner, WinApi};

mod handler;
pub use handler::get_handler;

pub type Context = AsnContext<WinApi>;
pub type Scene = AsnScene;

pub fn get_context() -> (Runner, Context, Scene) {
    let runner = Runner::new();
    let winapi = runner.new_winapi();
    let scene = AsnScene::new();
    let ctx = Context::new(winapi);
    (runner, ctx, scene)
}
