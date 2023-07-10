use crate::library::winapi_context::WinapiContext;
use crate::library::winapi_trait::WinApiTrait;

pub struct Context {
    winapi: WinapiContext,
}

impl Context {
    pub fn new(winapi: WinapiContext) -> Self {
        Self { winapi }
    }
    pub fn get_winapi(&mut self) -> &mut WinapiContext {
        &mut self.winapi
    }
}
