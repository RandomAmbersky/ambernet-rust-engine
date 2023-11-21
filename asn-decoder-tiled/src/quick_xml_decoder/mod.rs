use crate::quick_xml_decoder::tileset::DecodedTileset;
use quick_xml::de::from_str;
use quick_xml::se::to_string;

mod image;
mod test_serde;
mod tile;
mod tileset;

pub fn from_xml(s: &str) -> DecodedTileset {
    from_str(s).unwrap()
}

pub fn to_xml(t: &DecodedTileset) -> String {
    to_string(&t).unwrap()
}
