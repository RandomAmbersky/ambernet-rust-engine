use asn_core::{Array2D, Point2D, Size2D};

pub fn set_view_from(src: &Array2D, pos: &Point2D, window: &Size2D, dst: &mut Array2D) -> Result<(), String> {

    let mut bytes: Vec<u8> = Vec::new();

    let mut index = src.get_ingex(pos.x, pos.y)?;

    for _ in 0..window.height{
        for c_x in index..index + window.width as usize {
            bytes.push(src.bytes[c_x]);
        }
        index += window.width as usize;
    }

    dst.width = window.width;
    dst.height = window.height;
    dst.bytes = bytes;

    Ok(())
}

pub fn look_at(src: &Array2D, pos: &Point2D, window: &Size2D, dst: &mut Array2D) -> Result<(), String> {
    let half_width = window.width / 2;
    let half_height = window.height / 2;

    let map_width_minus_half_width = src.width - half_width;
    let map_height_minus_half_height = src.height - half_height;

    let mut n_pos: Point2D = *pos;

    if n_pos.x < half_width {
        n_pos.x = 0;
    } else if n_pos.x > map_width_minus_half_width {
        n_pos.x = map_width_minus_half_width;
    } else {
        n_pos.x -= half_width;
    }

    if n_pos.y < half_height {
        n_pos.y = 0;
    } else if n_pos.y > map_height_minus_half_height {
        n_pos.y = map_height_minus_half_height;
    } else {
        n_pos.y -= half_height;
    }

    set_view_from(src, &n_pos, window, dst)
}
