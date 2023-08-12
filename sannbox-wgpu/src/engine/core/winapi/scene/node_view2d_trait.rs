use crate::engine::core::errors::AsnRenderError;
use crate::engine::core::math::{Pos2D, Size2D};
use crate::engine::core::traits::TAsnWinapi;
use crate::engine::core::winapi::scene::node_base_trait::TNodeBase;
use crate::engine::core::winapi::AsnTextureFormat;

pub trait TNodeView2d: TNodeBase {
    type WinApi: TAsnWinapi;
    type CellType;
    fn set_tile_texture(
        &mut self,
        gfx: &mut Self::WinApi,
        bytes: &[u8],
        f: AsnTextureFormat,
    ) -> Result<(), AsnRenderError>;
    fn set_view_size(&mut self, size: Size2D<u32>) -> Result<(), AsnRenderError>;
    fn set_cell(pos: Pos2D<u32>, c: Self::CellType) -> Result<(), AsnRenderError>;
}
