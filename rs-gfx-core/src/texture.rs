use crate::AsnTextureFormat;
use crate::GfxContextTrait;
use rs_core::AsnError;

pub trait AsnTextureTrait<T, G, E, A>
where
    G: GfxContextTrait<E>,
    E: AsnError,
{
    fn from_raw_image(gfx: &G, bytes: &[u8], f: AsnTextureFormat) -> Result<T, E>;
    fn from_array(gfx: &G, array: &A, f: AsnTextureFormat) -> Result<T, E>;

    fn update_from_array(&mut self, gfx: &G, array: &A) -> Result<(), E>;
}
