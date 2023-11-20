use serde::{Deserialize, Serialize};

use crate::quick_xml_decoder::image::{image_deserialize, image_serialize, Image};
use crate::quick_xml_decoder::tile::Tile;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "tileset", rename_all = "lowercase")]
pub struct TileSet {
    #[serde(rename = "@version")]
    version: String,
    #[serde(rename = "@tiledversion")]
    tiledversion: String,
    #[serde(serialize_with = "image_serialize")]
    #[serde(deserialize_with = "image_deserialize")]
    image: Image,
    tile: Vec<Tile>,
}
