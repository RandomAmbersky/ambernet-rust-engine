use amberskynet_logger_web::LoggerWeb;
use crate::utils::{MAP_XML, parse_declaration};
use xmlparser::{Token, Tokenizer};

mod utils;

fn load_layer(iter: &mut Tokenizer) {
	LoggerWeb::log("Parse layer:");
	loop {
		let result = iter.next();
		if result.is_none() {
			LoggerWeb::log("End of parsed layer");
			return;
		}
		let token = result.unwrap().unwrap();
		let str = format!("Token: {:?}", token);
		LoggerWeb::log(&str);
	}
}

fn load_map(iter: &mut Tokenizer) {
	LoggerWeb::log("Parse map:");

	loop {
		let result = iter.next();
		if result.is_none() {
			LoggerWeb::log("End of parsed map");
			return;
		}
		let token = result.unwrap().unwrap();
		match token {
			Token::ElementStart { local, .. } => {
				if local.as_str() == "layer" {
					load_layer(iter);
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
	loop {
		let result = iter.next();
		if result.is_none() {
			LoggerWeb::log("End of xml file");
			return;
		}

		let token: Token = result.unwrap().unwrap();
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
