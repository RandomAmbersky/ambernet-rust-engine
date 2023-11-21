use serde::{Deserialize, Serialize};

use crate::quick_xml_decoder::image::{image_deserialize, image_serialize, Image};
use crate::quick_xml_decoder::tile::Tile;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "tileset", rename_all = "lowercase")]
pub struct DecodedTileset {
    #[serde(rename = "@version")]
    version: String,
    #[serde(rename = "@tiledversion")]
    tiledversion: String,
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@tilewidth")]
    tilewidth: u32,
    #[serde(rename = "@tileheight")]
    tileheight: u32,
    #[serde(rename = "@tilecount")]
    tilecount: u32,
    #[serde(rename = "@columns")]
    columns: u32,
    #[serde(serialize_with = "image_serialize")]
    #[serde(deserialize_with = "image_deserialize")]
    image: Image,
    tile: Vec<Tile>,
}

// pub struct DecodedTileset {
//     pub name: String,
//     pub tile_size: Size2D<u32>,
//     pub tile_count: u32,
//     pub columns: u32,
//     pub image_info: DecodedImageInfo,
//     pub tiles: Vec<DecodedTileInfo>
// }
