use crate::engine::core::errors::AsnRenderError;
use crate::engine::core::winapi::{AsnTextureFormat, TAsnWinapi};

pub trait TTexture {
	type WinApi: TAsnWinapi;
	type AsnTexture: TTexture;
	fn from_memory(gfx: &Self::WinApi, bytes: &[u8], f: AsnTextureFormat) -> Result<Self::AsnTexture, AsnRenderError>;
}
