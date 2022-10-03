use decoded_map::{DecodedMap};

mod decoded_map;

mod utils;
mod loader_tmx;
mod loader_tsx;

pub fn load_tmx(buf: &[u8]) -> Result<DecodedMap, String> {
	let map_str = match std::str::from_utf8(buf) {
		Ok(v) => v,
		Err(err) => {
			return Err(err.to_string())
		},
	};
	let map = loader_tmx::parse(map_str)?;
	Ok(map)
}

// pub fn load_tsx(buf: &[u8]) -> Result<DecodedMap, String> {
// 	let map_str = match std::str::from_utf8(buf) {
// 		Ok(v) => v,
// 		Err(err) => {
// 			return Err(err.to_string())
// 		},
// 	};
// 	let tileset = loader_tsx::parse(map_str)?;
// 	Ok(tileset)
// }
