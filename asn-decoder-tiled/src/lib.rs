mod loader_tmx;
mod loader_tsx;
mod test_serde;
mod utils;

use loader_tmx::DecodedMap;
use loader_tsx::DecodedTileset;
pub use test_serde::load_serde_tsx;

pub fn load_tmx(buf: &[u8]) -> Result<DecodedMap, String> {
    let map_str = match std::str::from_utf8(buf) {
        Ok(v) => v,
        Err(err) => return Err(err.to_string()),
    };
    let map = loader_tmx::parse(map_str)?;
    Ok(map)
}

pub fn load_tsx(buf: &[u8]) -> Result<DecodedTileset, String> {
    let map_str = match std::str::from_utf8(buf) {
        Ok(v) => v,
        Err(err) => return Err(err.to_string()),
    };
    let tileset = loader_tsx::parse(map_str)?;
    Ok(tileset)
}
