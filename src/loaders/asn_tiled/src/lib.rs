mod utils;
mod loader;

use asn_core::Array2D;

pub fn load_xml_map (buf: &[u8]) -> Result<Array2D, String> {
	let map_str = match std::str::from_utf8(buf) {
		Ok(v) => v,
		Err(err) => {
			return Err(err.to_string())
		},
	};
	let map = loader::parse(map_str)?;
	Ok(map)
}
