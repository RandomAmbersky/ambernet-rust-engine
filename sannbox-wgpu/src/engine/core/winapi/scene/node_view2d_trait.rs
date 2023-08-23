use crate::engine::core::errors::AsnRenderError;
use crate::engine::core::math::{Pos2D, Size2D, UnsignedNum};
use crate::engine::core::winapi::scene::node_base_trait::TNodeBase;

pub trait TNodeView2d: TNodeBase {
    type CellType;
    type SizeDimension: UnsignedNum;
    fn new(gfx: &mut Self::WinApi, tile_texture: &Self::AsnTexture, size: &Size2D<u32>) -> Self;
    fn set_cell(
        &mut self,
        pos: &Pos2D<Self::SizeDimension>,
        c: Self::CellType,
    ) -> Result<(), AsnRenderError>;
}
