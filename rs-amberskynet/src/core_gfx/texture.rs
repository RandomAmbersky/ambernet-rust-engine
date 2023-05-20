use crate::core_gfx::gfx_context::GfxContextTrait;
use rs_core::{Array2D, AsnError, Size2D};

pub trait AsnTextureTrait<T, G, E, A>
where
    G: GfxContextTrait<E>,
    E: AsnError,
{
    fn from_raw_image(gfx: &G, bytes: &[u8]) -> Result<T, E>;
    fn from_image(gfx: &G, img: &image::DynamicImage) -> Result<T, E>;
    fn from_array(gfx: &G, array: &A) -> Result<T, E>;

    fn update_from_array(&mut self, gfx: &G, array: &A) -> Result<(), E>;

    // fn get_size(&self) -> S;
}
