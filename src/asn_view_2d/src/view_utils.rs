use amberskynet_logger_web::LoggerWeb;
use asn_core::{Array2D, Point2D, Size2D};

pub fn set_view_from(src: &Array2D, pos: &Point2D, window: &Size2D, dst: &mut Array2D) -> Result<(), String> {

    // let mess = format!("src {:?} x {:?}", src.width, src.height);
    // LoggerWeb::log(&mess);

    if window.is_zero() {
        return Err(String::from("window size is zero"))
    }

    let mut bytes: Vec<u8> = Vec::new();

    let mut index = src.get_ingex(pos)?;

    for _ in 0..window.height {
        let max_index = index + window.width as usize;
        // let mess = format!("set_view_from {:?} {:?} {:?}", window, index, max_index);
        // LoggerWeb::log(&mess);
        for c_x in index..max_index {
            bytes.push(src.bytes[c_x]);
            // let mess = format!("index {:?} {:?}", c_x, src.get_pos(c_x)?);
            // LoggerWeb::log(&mess);
        }
        index += src.width as usize;
    }

    dst.width = window.width;
    dst.height = window.height;
    dst.bytes = bytes;

    Ok(())
}

pub fn look_at(src: &Array2D, pos: &Point2D, window: &Size2D, dst: &mut Array2D) -> Result<(), String> {

    // let mess = format!("look_at: {:?} {:?}", pos, window);
    // LoggerWeb::log(&mess);

    let half_width = window.width / 2;
    let half_height = window.height / 2;

    let map_width_minus_width = src.width - window.width;
    let map_height_minus_height = src.height - window.height;

    let mut n_pos: Point2D = *pos;

    if n_pos.x > half_width {
        n_pos.x -= half_width;
    } else {
        n_pos.x = 0;
    }

    if n_pos.y > half_height {
        n_pos.y -= half_height;
    } else {
        n_pos.y = 0;
    }

    if n_pos.y > map_height_minus_height {
        n_pos.y = map_height_minus_height;
    }

    if n_pos.x > map_width_minus_width {
        n_pos.x = map_width_minus_width;
    }

    // let mess = format!("n_pos: {:?}", n_pos);
    // LoggerWeb::log(&mess);

    set_view_from(src, &n_pos, window, dst)
}
