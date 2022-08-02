use xmlparser::{StrSpan, Token};
use amberskynet_logger_web::LoggerWeb;

pub const MAP_XML: &'static [u8] = include_bytes!("./laboratory3.tmx");
pub const MAP_JSON: &'static [u8] = include_bytes!("./laboratory3.json");

pub fn is_start(token: &Token, tag: &str) -> bool {
	if let Token::ElementStart { local, .. } = token {
		return local.as_str() == tag
	}
	false
}
