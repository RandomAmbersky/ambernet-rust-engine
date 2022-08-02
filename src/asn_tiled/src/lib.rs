use amberskynet_logger_web::LoggerWeb;
use crate::utils::{MAP_XML, is_start, is_end};
use xmlparser::{Token, Tokenizer};

mod utils;

#[derive(Default, Debug)]
struct LoadedMap {
	width: Option<i32>,
	height: Option<i32>,
	tile_width: Option<i32>,
	tile_height: Option<i32>
}

pub struct TiledLoader<'a> {
	parser: Tokenizer<'a>,
	loaded_map: LoadedMap
}

pub fn load_xml_map (_buf: &[u8]) {
	let map_str = match std::str::from_utf8(MAP_XML) {
		Ok(v) => v,
		Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
	};
	let mut loader = TiledLoader::new(map_str);
	loader.parse()
}

impl<'a> TiledLoader<'a> {
	pub fn new (text: &str) -> TiledLoader {
		TiledLoader {
			parser: Tokenizer::from(text),
			loaded_map: LoadedMap::default()
		}
	}

	// pub fn next_token(&mut self) -> Option<Result<Token, Error>> {
	// 	self.parser.next()
	// }

	fn parse_data_text(&mut self, map: &str) {
		let str = format!("data text: {}", map);
		LoggerWeb::log(&str);
	}

	fn parse_data(&mut self) {
		LoggerWeb::log("data start");
		while let Some(result) = self.parser.next() {
			let token = result.unwrap();
			if is_end(&token, "data") {
				LoggerWeb::log("layer end");
				return;
			}
			else if let Token::Text { text } = token {
				self.parse_data_text( text.as_str() );
			}
		}
	}

	fn parse_layer(&mut self) {
		LoggerWeb::log("layer start");
		while let Some(result) = self.parser.next() {
			let token = result.unwrap();
			if is_start(&token, "data") {
				self.parse_data();
			}
			else if is_end(&token, "layer") {
				LoggerWeb::log("layer end");
				return;
			}
			else if let Token::Attribute { local, value,  .. } = token {
				let mess = format!("layer Attribute: {:?} = {:?}", local.as_str(), value.as_str() );
				LoggerWeb::log(&mess);
			}
		}
	}

	fn parse_map(&mut self) {
		LoggerWeb::log("map start");
		while let Some(result) = self.parser.next() {
			let token = result.unwrap();
			if is_end(&token, "map") {
				LoggerWeb::log("map end");
				return;
			}
			if is_start(&token, "layer") {
				self.parse_layer();
			} else if let Token::Attribute { local, value,  .. } = token {
				let mess = format!("map Attribute: {:?} = {:?}", local.as_str(), value.as_str() );
				if local.as_str() == "width" {
					self.loaded_map.width = Some(value.as_str().parse::<i32>().unwrap());
				}
				else if local.as_str() == "height" {
					self.loaded_map.height = Some(value.as_str().parse::<i32>().unwrap());
				}
				else if local.as_str() == "tilewidth" {
					self.loaded_map.tile_width = Some(value.as_str().parse::<i32>().unwrap());
				}
				else if local.as_str() == "tileheight" {
					self.loaded_map.tile_height = Some(value.as_str().parse::<i32>().unwrap());
				}
				LoggerWeb::log(&mess);
			}
		}
	}

	pub fn parse(&mut self) {
		while let Some(result) = self.parser.next() {
			let token = result.unwrap();
			if is_start(&token, "map") {
				self.parse_map();
			}
		}
		LoggerWeb::log("Parse ok");
		let str = format!("Map is: {:?}", self.loaded_map);
		LoggerWeb::log(&str);
	}
}
