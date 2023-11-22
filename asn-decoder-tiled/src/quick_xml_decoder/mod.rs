use crate::quick_xml_decoder::tileset::{DecodedTileset, DecodedTilesetInitial};
use quick_xml::de::from_str;
use quick_xml::se::to_string;

mod image;
mod test_serde;
mod tile;
mod tileset;

#[allow(dead_code)]
pub fn from_xml(s: &str) -> DecodedTileset {
    let initial: DecodedTilesetInitial = from_str(s).unwrap();
    initial.into()
}

#[allow(dead_code)]
pub fn to_xml(t: DecodedTileset) -> String {
    let initial: DecodedTilesetInitial = t.into();
    to_string(&initial).unwrap()
}
