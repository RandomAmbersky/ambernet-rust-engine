use asn_core::Array2D;

// из массива тайлов в текстуру для шейдера
pub fn from_array2d (buf: &Array2D, tiles_width: u8) -> Array2D {
    let mut bytes = Vec::new();

    for cell in buf.bytes.iter() {
        let index = cell - 1;
        let y = index / tiles_width;
        let x = index - y * tiles_width;

        bytes.push(x);
        bytes.push(y);
        bytes.push(255);
        bytes.push(255);
    }

    Array2D {
        width: buf.width,
        height: buf.height,
        bytes
    }
}
