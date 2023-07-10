mod context;
mod winapi_context;
mod winapi_trait;

use context::Context;
use winapi_context::WinapiContext;
pub use winapi_trait::WinApiTrait;

pub fn get_context() -> Context {
    let winapi = WinapiContext::new(640, 480);
    let ctx = Context::new(winapi);
    ctx
}
