use xmlparser::StrSpan;
use amberskynet_logger_web::LoggerWeb;

pub const MAP_XML: &'static [u8] = include_bytes!("./laboratory3.tmx");
pub const MAP_JSON: &'static [u8] = include_bytes!("./laboratory3.json");

pub fn parse_declaraion (version: StrSpan, encoding: Option<StrSpan>) {
	let mut str = String::new();

	let version = format!("Version: {:?}", version.as_str());
	str.push_str(&version);

	if encoding.is_some() {
		let encoding = format!(", encoding: {:?}", encoding.unwrap().as_str());
		str.push_str(&encoding);
	}

	LoggerWeb::log(&str);
}
