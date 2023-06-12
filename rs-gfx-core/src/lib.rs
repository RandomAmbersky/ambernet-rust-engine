mod gfx_context;
mod pixel;
mod texture;

pub use gfx_context::GfxContextTrait;
pub use pixel::PixelRGBA;
pub use texture::AsnTextureFormat;
pub use texture::AsnTextureTrait;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
