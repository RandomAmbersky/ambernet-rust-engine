use serde_derive::{Deserialize, Serialize};
use serde_xml_rs::from_str;

#[allow(dead_code)]
pub fn load_serde_tsx(buf: &[u8]) {
    let map_str = match std::str::from_utf8(buf) {
        Ok(v) => v,
        Err(err) => panic!("Error: {:?}", err.to_string()),
    };

    // #[serde(rename_all = "snake_case")]
    #[derive(Debug, Serialize, Deserialize)]
    struct TileSet {
        version: String,
        tiledversion: String,
        image: Image,
        tile: Vec<Tile>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Tile {
        id: String,
        #[serde(rename = "type")]
        _type: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Image {
        source: String,
        width: u16,
        height: u16,
    }

    let item: TileSet = from_str(map_str).unwrap();
    panic!("TileSet: {:?} ", item);
}

#[cfg(test)]
mod tests {
    use crate::test_serde::load_serde_tsx;

    const MAP_TSX: &[u8] = include_bytes!("tiles.tsx");

    #[test]
    fn load_serde_tsx_ok() {
        load_serde_tsx(MAP_TSX);
    }
}
