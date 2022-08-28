use amberskynet_logger_web::LoggerWeb;

#[derive(Default, Debug)]
pub struct Map {
    pub width: u32,
    pub height: u32,
    pub cells: Vec<u8>
}

impl Map {
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
        let mess = format!("{} {} {} {} index is {} of {}", self.width, self.height, x, y, index, self.cells.len());
        LoggerWeb::log(&mess);
        self.cells[index as usize] = cell;
        Ok(())
    }

    pub fn get_ingex (&self, x: u32, y: u32) -> Result<usize, String> {
        let index: usize = (self.height * y + x) as usize;
        if self.cells.len() > index {
            return Ok(index as usize);
        }
        let mess = format!("Invalid index {} on map [{},{}]", index, x, y);
        Err(mess)
    }
}
