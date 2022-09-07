use asn_core::Array2D;

pub fn decode_texture (
    buf: &[u8]
) -> Result<Array2D, String> {
    let img = match image::load_from_memory(buf) {
        Ok(t) => t,
        Err(why) => {
            let err = format!("image::load_from_memory error: {}", why);
            return Err(err);
        }
    };
    let resp = Array2D {
        width: img.width(),
        height: img.height(),
        bytes: img.to_rgba8().into_raw()
    };

    Ok(resp)
}
