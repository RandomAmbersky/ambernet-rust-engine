mod utils;
mod loader;
mod map;

use map::Map;
use amberskynet_logger_web::LoggerWeb;

pub fn load_xml_map (buf: &[u8]) -> Result<Map, String> {
	let map_str = match std::str::from_utf8(buf) {
		Ok(v) => v,
		Err(err) => {
			return Err(err.to_string())
		},
	};
	let map = loader::parse(map_str)?;
	Ok(map)
}
