mod gfx_context;
mod pixel;
mod texture;
mod texture_format;

pub use gfx_context::GfxContextTrait;
pub use pixel::PixelRGBA;
pub use texture::AsnTextureTrait;
pub use texture_format::AsnTextureFormat;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
