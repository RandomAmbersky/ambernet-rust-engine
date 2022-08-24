pub struct DecodedTexture {
    pub width: i32,
    pub height: i32,
    pub bytes: Vec<u8>
}

pub fn decode_texture (
    buf: &[u8]
) -> Result<DecodedTexture, String> {
    let img = match image::load_from_memory(buf) {
        Ok(t) => t,
        Err(why) => {
            let err = format!("image::load_from_memory error: {}", why);
            return Err(err);
        }
    };
    let resp = DecodedTexture {
        width: img.width() as i32,
        height: img.height() as i32,
        bytes: img.to_rgba8().into_raw()
    };

    Ok(resp)
}
