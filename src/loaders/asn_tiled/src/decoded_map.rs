#[derive(Default, Debug)]
pub struct DecodedMap {
    pub width: u32,
    pub height: u32,
    pub tile_width: u32,
    pub tile_height: u32,
    pub map: Vec<u8>
}
