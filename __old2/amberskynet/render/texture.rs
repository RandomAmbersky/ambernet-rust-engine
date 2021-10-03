use web_sys::WebGlTexture;

pub struct Texture {
    pub texture: WebGlTexture,
    pub width: u32,
    pub height: u32,
}
