use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};
// use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct DataLayerMapSet {
    #[serde(rename = "@encoding")]
    encoding: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LayerMapSet {
    data: DataLayerMapSet,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapTileSet {
    #[serde(rename = "@source")]
    source: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapSet {
    #[serde(rename = "@version")]
    version: String,
    #[serde(rename = "@tiledversion")]
    tiledversion: String,
    #[serde(rename = "@orientation")]
    orientation: String,
    #[serde(rename = "@width")]
    width: u32,
    #[serde(rename = "@height")]
    height: u32,
    #[serde(rename = "@tilewidth")]
    tilewidth: u32,
    #[serde(rename = "@tileheight")]
    tileheight: u32,
    tileset: MapTileSet,
    #[serde(rename = "layer")]
    layers: Vec<LayerMapSet>,
}

#[allow(dead_code)]
pub fn load_map(buf: &[u8]) -> MapSet {
    let map_str = match std::str::from_utf8(buf) {
        Ok(v) => v.replace('\n', "").replace("> <", "><"),
        Err(err) => panic!("Error: {:?}", err.to_string()),
    };
    let res: MapSet = from_str(&map_str).unwrap();
    res
}
