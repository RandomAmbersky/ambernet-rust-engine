use std::any::Any;
use amberskynet_logger_web::LoggerWeb;
use crate::utils::{MAP_XML, parse_declaraion};
use xmlparser::{Token, Tokenizer};

mod utils;

pub fn load_xml_map (_buf: &[u8]) {
	let map_str = match std::str::from_utf8(MAP_XML) {
		Ok(v) => v,
		Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
	};
	for raw_token in Tokenizer::from(map_str) {
		let token: Token = raw_token.unwrap();
		match token {
			Token::Declaration { version, encoding, .. } => {
				parse_declaraion(version, encoding);
			}
			Token::ProcessingInstruction { .. } => {}
			Token::Comment { .. } => {}
			Token::DtdStart { .. } => {}
			Token::EmptyDtd { .. } => {}
			Token::EntityDeclaration { .. } => {}
			Token::DtdEnd { .. } => {}
			Token::ElementStart { prefix, local, span } => {
				let str = format!("ElementStart: {} {} {}", prefix.as_str(), local.as_str(), span.as_str());
				LoggerWeb::log(&str);
			}
			Token::Attribute { .. } => {}
			Token::ElementEnd { .. } => {}
			Token::Text { .. } => {}
			Token::Cdata { .. } => {}
		};
		// let str = format!("{:?}", token);
		// LoggerWeb::log(&str);
	}
}
