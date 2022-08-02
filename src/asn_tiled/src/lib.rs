use amberskynet_logger_web::LoggerWeb;
use crate::utils::{MAP_XML, is_start, is_end};
use xmlparser::{Token, Tokenizer};

mod utils;

#[derive(Default, Debug)]
struct LoadedMap {
	width: Option<i32>,
	height: Option<i32>,
	tile_width: Option<i32>,
	tile_height: Option<i32>,
	map: Vec<i32>
}

#[derive(Default, Debug)]
pub struct Map {
	width: i32,
	height: i32,
	tile_width: i32,
	tile_height: i32,
	map: Vec<i32>
}

impl LoadedMap {
	fn unwrap (&self) -> Map {
		let map = Map {
			width: self.width.unwrap(),
			height: self.height.unwrap(),
			tile_width: self.tile_width.unwrap(),
			tile_height: self.tile_height.unwrap(),
			map: self.map.to_vec()
		};
		map
	}
}

pub struct TiledLoader<'a> {
	parser: Tokenizer<'a>,
	loaded_map: LoadedMap
}

pub fn load_xml_map (_buf: &[u8]) -> Map {
	let map_str = match std::str::from_utf8(MAP_XML) {
		Ok(v) => v,
		Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
	};
	let mut loader = TiledLoader::new(map_str);
	let map = loader.parse();
	map
}

impl<'a> TiledLoader<'a> {
	pub fn new (text: &str) -> TiledLoader {
		TiledLoader {
			parser: Tokenizer::from(text),
			loaded_map: LoadedMap::default()
		}
	}

	fn parse_data_text(&mut self, map: &str) {
		self.loaded_map.map = Vec::new();
		for x in map.replace('\n', "").split(',') {
			let cell_num = x.parse::<i32>().unwrap();
			self.loaded_map.map.push(cell_num);
		};
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
			// else if let Token::Attribute { local, value,  .. } = token {
			// 	let mess = format!("layer Attribute: {:?} = {:?}", local.as_str(), value.as_str() );
			// 	LoggerWeb::log(&mess);
			// }
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
				// let mess = format!("map Attribute: {:?} = {:?}", local.as_str(), value.as_str() );
				// LoggerWeb::log(&mess);
			}
		}
	}

	pub fn parse(&mut self) -> Map {
		while let Some(result) = self.parser.next() {
			let token = result.unwrap();
			if is_start(&token, "map") {
				self.parse_map();
			}
		}
		LoggerWeb::log("Parse ok");
		// let str = format!("Map is: {:?}", self.loaded_map);
		// LoggerWeb::log(&str);
		self.loaded_map.unwrap()
	}
}
