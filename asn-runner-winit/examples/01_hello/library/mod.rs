use asn_core::AsnContext;
use asn_runner_winit::asn_winapi::AsnWgpuWinApi;

pub mod handler;

pub type Context = AsnContext<AsnWgpuWinApi>;

pub fn get_context() -> Context {
    let winapi = AsnWgpuWinApi::new();
    let ctx = Context::new(winapi);
    ctx
}
