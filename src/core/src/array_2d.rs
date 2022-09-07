use crate::{Point2D, Size2D};

#[derive(Default, Debug)]
pub struct Array2D {
    pub width: u32,
    pub height: u32,
    pub bytes: Vec<u8>
}

impl Array2D {
    pub fn set_cell (&mut self,  point: &Point2D, cell: u8) -> Result<(), String> {
        if point.x > self.width {
            let mess = format!("Invalid width {}", point.x);
            return Err(mess);
        }
        if point.y > self.height {
            let mess = format!("Invalid height {}", point.y);
            return Err(mess);
        }
        let index = self.get_ingex(point)?;

        self.bytes[index as usize] = cell;
        Ok(())
    }

    pub fn get_ingex (&self, point: &Point2D) -> Result<usize, String> {
        let index: usize = (self.width * point.y + point.x) as usize;
        if self.bytes.len() > index {
            return Ok(index as usize);
        }
        let mess = format!("Invalid index {} on map [{},{}]", index, point.x, point.y);
        Err(mess)
    }

    pub fn get_pos (&self, index: usize) -> Result<Point2D, String> {
        let y = index / (self.width as usize);
        let x = index - y * (self.width as usize);
        let pos = Point2D {
            x: x as u32,
            y: y as u32
        };
        Ok(pos)
    }

    pub fn get_cell(&self, point: &Point2D) -> Result<u8, String> {
        if point.x > self.width {
            let mess = format!("Invalid width {}", point.x);
            return Err(mess);
        }
        if point.y > self.height {
            let mess = format!("Invalid height {}", point.y);
            return Err(mess);
        }
        let index = self.get_ingex(point)?;
        Ok(self.bytes[index])
    }
}
