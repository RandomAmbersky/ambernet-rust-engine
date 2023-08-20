use crate::engine::core::errors::AsnRenderError;
use crate::engine::core::winapi::scene::node_base_trait::TNodeBase;
use std::sync::Arc;

pub trait TNodeQuad: TNodeBase {
    fn set_texture(
        &mut self,
        gfx: &mut Self::WinApi,
        texture: &Self::AsnTexture,
    ) -> Result<(), AsnRenderError>;
}
