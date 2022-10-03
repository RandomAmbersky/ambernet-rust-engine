use asn_core::Size2D;

#[derive(Default, Debug)]
pub struct DecodedTileInfo {
    pub id: u32,
    pub class: String
}

#[derive(Default, Debug)]
pub struct DecodedImageInfo {
    pub source: String,
    pub size: Size2D,
}

#[derive(Default, Debug)]
pub struct DecodedTileset {
    pub name: String,
    pub tile_size: Size2D,
    pub tile_count: u32,
    pub columns: u32,
    pub image_info: DecodedImageInfo,
    pub tiles: Vec<DecodedTileInfo>
}

#[derive(Default, Debug)]
pub struct DecodedLayer {
    pub name: String,
    pub id: u32,
    pub size: Size2D,
    pub bytes: Vec<u8>,
    pub visible: bool
}

#[derive(Default, Debug)]
pub struct DecodedMap {
    pub size: Size2D,
    pub tilesets: Vec<DecodedTileset>,
    pub layers: Vec<DecodedLayer>
}
