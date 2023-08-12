use crate::engine::core::errors::AsnRenderError;
use crate::engine::core::math::{Pos2D, Size2D};
use crate::engine::core::traits::TAsnWinapi;
use crate::engine::core::winapi::scene::node_base_trait::TNodeBase;
use crate::engine::core::winapi::AsnTextureFormat;

pub trait TNodeView2d: TNodeBase {
    type WinApi: TAsnWinapi;
    type Size2d;
    type CellSize;
    type Position;
    fn set_tile_texture(
        &mut self,
        gfx: &mut Self::WinApi,
        bytes: &[u8],
        f: AsnTextureFormat,
    ) -> Result<(), AsnRenderError>;
    fn set_view_size(&mut self, size: Self::Size2d) -> Result<(), AsnRenderError>;
    fn set_cell(pos: Self::Position, c: Self::CellSize) -> Result<(), AsnRenderError>;
}
