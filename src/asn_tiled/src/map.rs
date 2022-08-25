#[derive(Default, Debug)]
pub struct Map {
    pub width: i32,
    pub height: i32,
    pub tile_width: i32,
    pub tile_height: i32,
    pub map: Vec<u8>
}
