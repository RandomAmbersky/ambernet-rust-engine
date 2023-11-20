use crate::quick_xml_decoder::tileset::TileSet;
use quick_xml::de::from_str;
use quick_xml::se::to_string;

mod image;
mod test_serde;
mod tile;
mod tileset;

pub fn from_xml(s: &str) -> TileSet {
    from_str(s).unwrap()
}

pub fn to_xml(t: &TileSet) -> String {
    to_string(&t).unwrap()
}
