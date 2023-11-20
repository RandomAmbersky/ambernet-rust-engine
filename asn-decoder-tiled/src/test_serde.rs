use asn_core::math::Size2D;
use quick_xml::de::from_str;
use quick_xml::se::to_string;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "tileset", rename_all = "lowercase")]
struct TileSet {
    #[serde(rename = "@version")]
    version: String,
    #[serde(rename = "@tiledversion")]
    tiledversion: String,
    #[serde(serialize_with = "image_serialize")]
    #[serde(deserialize_with = "image_deserialize")]
    image: Image,
    tile: Vec<Tile>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
struct ImageInitial {
    #[serde(rename = "@source")]
    source: String,
    #[serde(rename = "@width")]
    width: u32,
    #[serde(rename = "@height")]
    height: u32,
}

pub fn image_deserialize<'de, D>(deserializer: D) -> Result<Image, D::Error>
where
    D: Deserializer<'de>,
{
    let initial: ImageInitial = ImageInitial::deserialize(deserializer)?;

    Ok(Image {
        source: initial.source,
        size: Size2D {
            width: initial.width,
            height: initial.height,
        },
    })
}

pub fn image_serialize<S>(image: &Image, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let initial: ImageInitial = ImageInitial {
        source: image.source.clone(),
        width: image.size.width,
        height: image.size.height,
    };

    initial.serialize(serializer)

    // let mut my_struct = serializer.serialize_map(None)?;
    // my_struct.serialize_entry("source", &image.source)?;
    // my_struct.serialize_entry("width", &image.size.width)?;
    // my_struct.serialize_entry("height", &image.size.height)?;
    // my_struct.end()
}

#[derive(Debug, Serialize, Deserialize)]
struct Tile {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@type")]
    _type: String,
}

#[derive(Debug, Default)]
pub struct Image {
    pub source: String,
    pub size: Size2D<u32>,
}

#[allow(dead_code)]
pub fn load_serde_tsx(buf: &[u8]) {
    let map_str = match std::str::from_utf8(buf) {
        Ok(v) => v,
        Err(err) => panic!("Error: {:?}", err.to_string()),
    };

    let item: TileSet = from_str(map_str).unwrap();
    let str = to_string(&item).unwrap();

    let item: TileSet = from_str(map_str).unwrap();
    panic!("{:?} \n {:?}", map_str, str);
    // panic!("TileSet: {:?}", item);
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
