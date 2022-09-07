use amberskynet_logger_web::LoggerWeb;
use asn_core::{Point2D, Size2D};

#[derive(Default, Debug)]
pub struct Array2D {
    pub width: u32,
    pub height: u32,
    pub bytes: Vec<u8>
}

impl Array2D {
    pub fn set_cell (&mut self,  x: u32, y: u32, cell: u8) -> Result<(), String> {
        if x > self.width {
            let mess = format!("Invalid width {}", x);
            return Err(mess);
        }
        if y > self.height {
            let mess = format!("Invalid height {}", y);
            return Err(mess);
        }
        let index = self.get_ingex(x, y)?;

        self.bytes[index as usize] = cell;
        Ok(())
    }

    pub fn get_ingex (&self, x: u32, y: u32) -> Result<usize, String> {
        let index: usize = (self.height * y + x) as usize;
        if self.bytes.len() > index {
            return Ok(index as usize);
        }
        let mess = format!("Invalid index {} on map [{},{}]", index, x, y);
        Err(mess)
    }

    pub fn get_cell(&self, x: u32, y: u32) -> Result<u8, String> {
        if x > self.width {
            let mess = format!("Invalid width {}", x);
            return Err(mess);
        }
        if y > self.height {
            let mess = format!("Invalid height {}", y);
            return Err(mess);
        }
        let index = self.get_ingex(x, y)?;
        Ok(self.bytes[index as usize])
    }

    pub fn set_view_from(&mut self, pos: &Point2D, window: &Size2D, map: &Array2D) -> Result<(), String> {

        let mut bytes: Vec<u8> = Vec::new();

        let mut index = map.get_ingex(pos.x, pos.y)?;

        for _ in 0..window.height{
            for c_x in index..index + window.width as usize {
                bytes.push(map.bytes[c_x]);
            }
            index += window.width as usize;
        }

        self.width = window.width;
        self.height = window.height;
        self.bytes = bytes;

        Ok(())
    }
}
