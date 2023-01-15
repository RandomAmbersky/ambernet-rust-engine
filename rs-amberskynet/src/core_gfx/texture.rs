use crate::core::{Array2D, AsnError, Size2D};
use crate::core_gfx::gfx_context::GfxContextTrait;
use crate::core_gfx::pixel::Pixel;
use crate::core_gfx::point2d::Point2D;

pub trait AsnTextureTrait<T, G, E, Pi, Po>
where
    G: GfxContextTrait<E>,
    E: AsnError,
{
    fn from_raw_image(gfx: &G, bytes: &[u8]) -> Result<T, E>;
    fn from_image(gfx: &G, img: &image::DynamicImage) -> Result<T, E>;
    fn from_array(gfx: &G, array: &Array2D<u32, u8>) -> Result<T, E>;

    fn update_from_array(&mut self, gfx: &G, array: &Array2D<u32, u8>) -> Result<(), E>;

    fn get_size(&self) -> Size2D<u32>;
}

pub trait AsnSurfaceTrait<U, S> {
    fn get_size(&self) -> Size2D<u32>;
    fn set_pixel(&mut self, pos: Point2D<U>, p: &Pixel<S>);
}
