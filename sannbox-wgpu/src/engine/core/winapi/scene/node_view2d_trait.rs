use std::sync::Arc;
use crate::engine::core::errors::AsnRenderError;
use crate::engine::core::math::{Pos2D, Size2D, UnsignedNum};
use crate::engine::core::winapi::scene::node_base_trait::TNodeBase;

pub trait TNodeView2d: TNodeBase {
    type CellType;
    type SizeDimension: UnsignedNum;
    fn set_tile_texture(
        &mut self,
        gfx: &mut Self::WinApi,
        texture: Arc<Self::AsnTexture>,
    ) -> Result<(), AsnRenderError>;
    fn set_view_size(&mut self, size: &Size2D<Self::SizeDimension>) -> Result<(), AsnRenderError>;
    fn set_cell(
        &mut self,
        pos: &Pos2D<Self::SizeDimension>,
        c: Self::CellType,
    ) -> Result<(), AsnRenderError>;
}
