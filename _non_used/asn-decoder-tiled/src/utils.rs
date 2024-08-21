use xmlparser::{Token};
use xmlparser::ElementEnd::Close;

pub fn is_start(token: &Token, tag: &str) -> bool {
	if let Token::ElementStart { local, .. } = token {
		return local.as_str() == tag
	}
	false
}

pub fn is_end(token: &Token, tag: &str) -> bool {
	if let Token::ElementEnd { end: Close(_, b), .. } = token {
		return b.as_str() == tag
	}
	false
}
