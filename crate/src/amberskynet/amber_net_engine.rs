use crate::amberskynet::render;
use crate::amberskynet::render::RenderContext;

pub struct AmberNetEngine {
 pub(crate) render_ctx: RenderContext
}

impl AmberNetEngine {
    pub fn new () -> Self {
        Self {
            render_ctx: render::render_ctx()
        }
    }
}
