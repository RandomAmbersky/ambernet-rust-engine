use crate::errors::AsnRenderError;
use crate::math::{Pos2D, Size2D, UnsignedNum};
use crate::winapi::scene::node_base_trait::TNodeBase;

pub trait TNodeView2d: TNodeBase {
    type CellType;
    type SizeDimension: UnsignedNum;
    fn new(
        gfx: &mut Self::WinApi,
        tile_texture: &Self::AsnTexture,
        view_size_in_tiles: &Size2D<u32>,
        tile_size_in_pixels: &Size2D<u32>,
    ) -> Self;
    fn set_cell(
        &mut self,
        pos: &Pos2D<Self::SizeDimension>,
        c: Self::CellType,
    ) -> Result<(), AsnRenderError>;
}
