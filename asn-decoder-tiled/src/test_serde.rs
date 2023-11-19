use quick_xml::de::from_str;
use quick_xml::se::to_string;
use serde::{Deserialize, Serialize};

// #[serde(rename_all = "snake_case")]
#[derive(Debug, Serialize, Deserialize)]
struct TileSet {
    #[serde(rename = "@version")]
    version: String,
    #[serde(rename = "@tiledversion")]
    tiledversion: String,
    // #[serde(serialize_with = "image_serialize")]
    // #[serde(deserialize_with = "image_deserialize")]
    image: ImageTmx,
    tile: Vec<Tile>,
}

// pub fn image_deserialize<'de, D>(deserializer: D) -> Result<ImageTmx, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     let buf = String::deserialize(deserializer)?;
//     println!("deserialize: {:?}", buf);
//     let image = ImageTmx::default();
//     Ok(image)
// }
//
// pub fn image_serialize<S>(image: &ImageTmx, serializer: S) -> Result<S::Ok, S::Error>
// where
//     S: Serializer,
// {
//     let mut my_struct = serializer.serialize_map(None)?;
//     my_struct.serialize_entry("source", &image.source)?;
//     my_struct.serialize_entry("width", &image.width)?;
//     my_struct.serialize_entry("height", &image.height)?;
//     my_struct.end()
// }

#[derive(Debug, Serialize, Deserialize)]
struct Tile {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@type")]
    _type: String,
}

// #[derive(Debug, Default)]
// pub struct Image {
//     source: String,
//     size: Size2D<u32>, // width: u16,
//                        // height: u16,
// }
//
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ImageTmx {
    #[serde(rename = "@source")]
    source: String,
    #[serde(rename = "@width")]
    width: u16,
    #[serde(rename = "@height")]
    height: u16,
}

#[allow(dead_code)]
pub fn load_serde_tsx(buf: &[u8]) {
    let map_str = match std::str::from_utf8(buf) {
        Ok(v) => v,
        Err(err) => panic!("Error: {:?}", err.to_string()),
    };

    let item: TileSet = from_str(map_str).unwrap();
    // panic!("TileSet: {:?} ", item);

    let str = to_string(&item).unwrap();
    panic!("TileSet: {:?} string: {:?} ", item, str);
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
