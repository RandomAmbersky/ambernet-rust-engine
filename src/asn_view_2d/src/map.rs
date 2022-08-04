pub struct Map {
    pub width: i32,
    pub height: i32,
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
