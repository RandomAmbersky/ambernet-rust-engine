use crate::errors::AsnRenderError;
use crate::winapi::scene::node_base_trait::TNodeBase;

pub trait TNodeQuad: TNodeBase {
    fn set_texture(
        &mut self,
        gfx: &mut Self::WinApi,
        texture: &Self::AsnTexture,
    ) -> Result<(), AsnRenderError>;
}
