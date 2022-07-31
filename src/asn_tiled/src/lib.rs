use amberskynet_logger_web::LoggerWeb;
use crate::utils::{MAP_XML, parse_declaration};
use xmlparser::{Token, Tokenizer};

mod utils;

fn load_layer(iter: &mut Tokenizer) {
	LoggerWeb::log("Parse layer:");
	while let Some(result) = iter.next() {
		let token = result.unwrap();
		let str = format!("Token: {:?}", token);
		LoggerWeb::log(&str);
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
