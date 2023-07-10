mod context;
mod winapi_context;

use context::Context;
use winapi_context::WinapiContext;

pub fn get_context() -> Context {
    let winapi = WinapiContext::new(640, 480);
    let ctx = Context::new(winapi);
    ctx
}
