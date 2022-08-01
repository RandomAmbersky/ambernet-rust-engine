use amberskynet_logger_web::LoggerWeb;
use crate::utils::{MAP_XML, parse_declaration};
use xmlparser::{Token, Tokenizer};

mod utils;

struct LoadedMap {
	width: Option<i32>,
	height: Option<i32>
}

fn load_data(iter: &mut Tokenizer) {
	for result in iter.by_ref() {
		let str = format!("data: {:?}", result.unwrap());
		LoggerWeb::log(&str);
	}
}

fn load_layer(iter: &mut Tokenizer) {
	LoggerWeb::log("Parse layer:");
	let mut map = LoadedMap {
		width: None,
		height: None
	};
	while let Some(result) = iter.next() {
		let token = result.unwrap();
		match token {
			Token::ElementStart { local, .. } => {
				if local.as_str() == "data" {
					load_data(iter);
				}
			},
			Token::Attribute { local, value, .. } => {
				if local.as_str() == "width" {
					map.width = Some(value.as_str().parse::<i32>().unwrap());
				}
				if local.as_str() == "height" {
					map.height = Some(value.as_str().parse::<i32>().unwrap());
				}
			},
			_ => {}
		}
	}
}

fn load_map(iter: &mut Tokenizer) {
	LoggerWeb::log("Parse map:");

	while let Some(result) = iter.next() {
		let token = result.unwrap();
		match token {
			Token::ElementStart { local, .. } => {
				if local.as_str() == "layer" {
					load_layer(iter);
					return; // only one layer
				}
			},
			_ => {}
		}
	}
}

pub fn load_xml_map (_buf: &[u8]) {
	let map_str = match std::str::from_utf8(MAP_XML) {
		Ok(v) => v,
		Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
	};
	let mut iter = Tokenizer::from(map_str);
	while let Some(result) = iter.next() {
		let token: Token = result.unwrap();
		match token {
			Token::Declaration {version, encoding, .. } => {
				parse_declaration(&version, &encoding);
			},
			Token::ElementStart { local, .. } => {
				if local.as_str() == "map" {
					load_map(&mut iter);
				}
			},
			_ => {
				let str = format!("Token: {:?}", token);
				LoggerWeb::log(&str);
			}
		}
	}
}
