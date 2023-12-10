use quick_xml::de;
use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TileInfo {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@type")]
    _type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TileSet {
    #[serde(rename = "@tilewidth")]
    pub tile_width: u32,
    #[serde(rename = "@tileheight")]
    pub tile_height: u32,
    #[serde(rename = "@tilecount")]
    pub tile_count: u32,
    #[serde(rename = "@columns")]
    pub columns: u32,
    #[serde(rename = "tile")]
    tiles: Vec<TileInfo>,
}

pub fn load_tileset(buf: &[u8]) -> TileSet {
    let map_str = match std::str::from_utf8(buf) {
        Ok(v) => v.replace('\n', "").replace("> <", "><"),
        Err(err) => panic!("Error: {:?}", err.to_string()),
    };
    let res: TileSet = from_str(&map_str).unwrap();
    res
}
