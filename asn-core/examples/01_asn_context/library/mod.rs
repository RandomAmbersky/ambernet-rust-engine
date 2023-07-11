mod winapi_context;

use asn_core::AsnContext;
use winapi_context::WinapiContext;

pub type Context = AsnContext<WinapiContext>;

pub fn get_context() -> Context {
    let winapi = WinapiContext::new(640, 480);
    let ctx = Context::new(winapi);
    ctx
}
