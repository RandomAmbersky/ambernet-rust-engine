pub struct Map {
    pub width: u32,
    pub height: u32,
    pub cells: Vec<u8>
}

impl Default for Map {
    fn default() -> Self {
        let cells: Vec<u8> = vec![
            1,1,1,1,1,1,1,1,1,1,
            1,0,0,0,0,0,0,0,0,1,
            1,0,0,0,0,0,0,0,0,1,
            1,0,0,0,0,0,0,0,0,1,
            1,0,0,0,0,0,0,0,0,1,
            1,0,0,0,0,0,0,0,0,1,
            1,0,0,0,0,0,0,0,0,1,
            1,0,0,0,0,0,0,0,0,1,
            1,0,0,0,0,0,0,0,0,1,
            1,1,1,1,1,1,1,1,1,1,
        ];
        Map {
            width: 10,
            height: 10,
            cells
        }
    }
}

impl Map {
    pub fn set_cell (&self,  x: u32, y: u32, cell: u8) -> Result<(), String> {
        if x > self.width {
            let mess = format!("Invalid width {}", x);
            return Err(mess);
        }
        if y > self.height {
            let mess = format!("Invalid height {}", y);
            return Err(mess);
        }
        let index = self.get_ingex(x, y);
        self.cells[index] = cell as u8;
        Ok(())
    }
    pub fn get_ingex (&self, x: u32, y: u32) -> usize {
        (self.height * y + x) as usize
    }
}
