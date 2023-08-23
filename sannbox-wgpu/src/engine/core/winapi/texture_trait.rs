use crate::engine::core::errors::AsnRenderError;
use crate::engine::core::math::Size2D;
use crate::engine::core::winapi::{AsnTextureFormat, TAsnWinapi};

pub trait TTexture {
    type WinApi: TAsnWinapi;
    type AsnTexture: TTexture;
    fn from_raw(
        gfx: &Self::WinApi,
        bytes: &[u8],
        size: &Size2D<u32>,
        f: AsnTextureFormat,
    ) -> Result<Self::AsnTexture, AsnRenderError>;

    fn get_size(&self) -> Size2D<u32>;

    fn update_from_raw(&mut self, gfx: &Self::WinApi, bytes: &[u8]) -> Result<(), AsnRenderError>;
}
