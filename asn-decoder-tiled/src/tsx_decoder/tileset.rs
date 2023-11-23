use asn_core::math::Size2D;
use serde::{Deserialize, Serialize};

use crate::tsx_decoder::image::{image_deserialize, image_serialize, DecodedImageInfo};
use crate::tsx_decoder::tile::DecodedTileInfo;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "tileset", rename_all = "lowercase")]
pub struct DecodedTilesetInitial {
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
    #[serde(rename = "image")]
    #[serde(serialize_with = "image_serialize")]
    #[serde(deserialize_with = "image_deserialize")]
    image_info: DecodedImageInfo,
    #[serde(rename = "tile")]
    tiles: Vec<DecodedTileInfo>,
}

impl Into<DecodedTileset> for DecodedTilesetInitial {
    fn into(self) -> DecodedTileset {
        DecodedTileset {
            version: self.version,
            tiledversion: self.tiledversion,
            name: self.name,
            tile_size: Size2D {
                width: self.tilewidth,
                height: self.tileheight,
            },
            tile_count: self.tilecount,
            columns: self.columns,
            image_info: self.image_info,
            tiles: self.tiles,
        }
    }
}

impl Into<DecodedTilesetInitial> for DecodedTileset {
    fn into(self) -> DecodedTilesetInitial {
        DecodedTilesetInitial {
            version: self.version,
            tiledversion: self.tiledversion,
            name: self.name,
            tilewidth: self.tile_size.width,
            tileheight: self.tile_size.height,
            tilecount: self.tile_count,
            columns: self.columns,
            image_info: self.image_info,
            tiles: self.tiles,
        }
    }
}

#[derive(Debug)]
pub struct DecodedTileset {
    version: String,
    tiledversion: String,
    pub name: String,
    pub tile_size: Size2D<u32>,
    pub tile_count: u32,
    pub columns: u32,
    pub image_info: DecodedImageInfo,
    pub tiles: Vec<DecodedTileInfo>,
}
