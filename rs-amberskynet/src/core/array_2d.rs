use crate::core::{Direction, Point2D, Size2D};

#[derive(Default, Debug)]
pub struct Array2D {
    pub size: Size2D,
    pub bytes: Vec<u8>,
}

impl Array2D {
    pub fn is_zero(&self) -> bool {
        return self.size.width == 0 && self.size.height == 0;
    }

    pub fn is_valid_pos(&self, pos: &Point2D) -> bool {
        return pos.x < self.size.width && pos.y < self.size.height;
    }

    pub fn move_point(&self, pos: &Point2D, dir: &Direction) -> Result<Point2D, String> {
        let dir_delta = dir.as_delta();
        let new_pos = pos.add(&dir_delta)?;
        if self.is_valid_pos(&new_pos) {
            return Ok(new_pos);
        }
        Err(String::from("can't move "))
    }

    pub fn set_cell(&mut self, point: &Point2D, cell: u8) -> Result<(), String> {
        if point.x > self.size.width {
            let mess = format!("Invalid width {}", point.x);
            return Err(mess);
        }
        if point.y > self.size.height {
            let mess = format!("Invalid height {}", point.y);
            return Err(mess);
        }
        let index = self.get_ingex(point)?;

        self.bytes[index as usize] = cell;
        Ok(())
    }

    pub fn get_ingex(&self, point: &Point2D) -> Result<usize, String> {
        let index: usize = (self.size.width * point.y + point.x) as usize;
        if self.bytes.len() > index {
            return Ok(index as usize);
        }
        let mess = format!("Invalid index {} on map [{},{}]", index, point.x, point.y);
        Err(mess)
    }

    pub fn get_pos(&self, index: usize) -> Result<Point2D, String> {
        let y = index / (self.size.width as usize);
        let x = index - y * (self.size.width as usize);
        let pos = Point2D {
            x: x as u32,
            y: y as u32,
        };
        Ok(pos)
    }

    pub fn get_cell(&self, point: &Point2D) -> Result<u8, String> {
        if point.x > self.size.width {
            let mess = format!("Invalid width {}", point.x);
            return Err(mess);
        }
        if point.y > self.size.height {
            let mess = format!("Invalid height {}", point.y);
            return Err(mess);
        }
        let index = self.get_ingex(point)?;
        Ok(self.bytes[index])
    }

    pub fn cut_from(&mut self, pos: &Point2D, src: &Array2D) -> Result<(), String> {
        let mut bytes: Vec<u8> = Vec::new();

        let mut index = src.get_ingex(&pos)?;

        for _ in 0..self.size.height {
            let max_index = index + self.size.width as usize;
            // let mess = format!("set_view_from {:?} {:?} {:?}", window, index, max_index);
            // LoggerWeb::log(&mess);
            for c_x in index..max_index {
                bytes.push(src.bytes[c_x]);
                // let mess = format!("index {:?} {:?}", c_x, src.get_pos(c_x)?);
                // LoggerWeb::log(&mess);
            }
            index += src.size.width as usize;
        }

        self.bytes = bytes;

        Ok(())
    }

    pub fn calc_screen_pos(&self, pos: &Point2D, screen: &Size2D) -> Result<Point2D, String> {
        if self.is_zero() {
            return Err(String::from("Array2D size is zero"));
        }

        if screen.is_zero() {
            return Err(String::from("screen size is zero"));
        }

        let half_width = screen.width / 2;
        let half_height = screen.height / 2;

        let map_width_minus_width = self.size.width - screen.width;
        let map_height_minus_height = self.size.height - screen.height;

        let mut n_pos = *pos;

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

        Ok(n_pos)
    }
}
