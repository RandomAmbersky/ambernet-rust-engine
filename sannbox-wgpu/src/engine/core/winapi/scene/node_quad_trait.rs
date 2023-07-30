use crate::engine::core::errors::AsnRenderError;
use crate::engine::core::traits::TAsnWinapi;
use crate::engine::core::winapi::scene::node_base_trait::TNodeBase;
use crate::engine::core::winapi::AsnTextureFormat;

pub trait TNodeQuad: TNodeBase {
    type WinApi: TAsnWinapi;
    fn set_texture(
        &mut self,
        gfx: &mut Self::WinApi,
        bytes: &[u8],
        f: AsnTextureFormat,
    ) -> Result<(), AsnRenderError>;
}
