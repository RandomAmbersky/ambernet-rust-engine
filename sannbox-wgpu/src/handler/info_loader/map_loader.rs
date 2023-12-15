use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};
// use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct MapSet {}

#[allow(dead_code)]
pub fn load_map(buf: &[u8]) -> MapSet {
    let map_str = match std::str::from_utf8(buf) {
        Ok(v) => v.replace('\n', "").replace("> <", "><"),
        Err(err) => panic!("Error: {:?}", err.to_string()),
    };
    let res: MapSet = from_str(&map_str).unwrap();
    res
}
