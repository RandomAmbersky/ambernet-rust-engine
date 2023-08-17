use crate::engine::core::errors::AsnRenderError;
use crate::engine::core::winapi::scene::node_base_trait::TNodeBase;

pub trait TNodeQuad: TNodeBase {
    fn set_texture(&mut self, texture: &Self::AsnTexture) -> Result<(), AsnRenderError>;
}
