use crate::library::handler::Handler;
use asn_core::AsnContext;
use asn_runner_winit::{AsnScene, Runner, WinApi};

mod handler;

pub type Context = AsnContext<WinApi>;
pub type Scene = AsnScene;

pub fn get_context() -> (Runner, Context, Scene) {
    let runner = Runner::new();
    let winapi = runner.new_winapi();
    let scene = AsnScene::new();
    let ctx = Context::new(winapi);
    (runner, ctx, scene)
}

pub fn get_handler() -> Handler {
    Handler::new()
}
