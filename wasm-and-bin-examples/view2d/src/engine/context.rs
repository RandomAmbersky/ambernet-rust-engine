use asn_core::AsnContext;
use asn_runner_winit::AsnWgpuWinApi;

pub type Context = AsnContext<AsnWgpuWinApi>;

pub fn get_context() -> Context {
    let runner = asn_runner_winit::new();
    Context::new(runner)
}
