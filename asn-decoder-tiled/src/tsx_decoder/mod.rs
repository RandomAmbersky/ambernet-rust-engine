use quick_xml::de::from_str;
use quick_xml::se::to_string;
use quick_xml::DeError;

mod image;
mod test_serde;
mod tile;
mod tileset;

use crate::tsx_decoder::tileset::DecodedTilesetInitial;
pub use tileset::DecodedTileset;

#[allow(dead_code)]
pub fn from_xml(s: &str) -> DecodedTileset {
    let initial: DecodedTilesetInitial = from_str(s).unwrap();
    initial.into()
}

#[allow(dead_code)]
pub fn to_xml(t: DecodedTileset) -> Result<String, String> {
    let initial: DecodedTilesetInitial = t.into();
    match to_string(&initial) {
        Ok(t) => Ok(t),
        Err(e) => Err(e.to_string()),
    }
}
