use xmlparser::{Token, Tokenizer};
use amberskynet_logger_web::LoggerWeb;
use asn_array_2d::Array2D;
use crate::utils::{is_end, is_start};

pub fn parse(text: &str) -> Result<Array2D, String> {
    let mut map = Array2D::default();

    let mut parser = Tokenizer::from(text);
    while let Some(result) = parser.next() {
        let token = result.unwrap();
        if is_start(&token, "map") {
            parse_map(&mut parser, &mut map);
        }
    }

    Ok(map)
}

fn parse_map(parser: &mut Tokenizer, map: &mut Array2D) {
    LoggerWeb::log("map start");
    while let Some(result) = parser.next() {
        let token = result.unwrap();
        if is_end(&token, "map") {
            LoggerWeb::log("map end");
            return;
        }
        if is_start(&token, "layer") {
            parse_layer(parser, map);
        } else if let Token::Attribute { local, value,  .. } = token {
            if local.as_str() == "width" {
                map.width = value.as_str().parse::<u32>().unwrap();
            }
            else if local.as_str() == "height" {
                map.height = value.as_str().parse::<u32>().unwrap();
            }
            // else if local.as_str() == "tilewidth" {
            //     map.tile_width = value.as_str().parse::<u32>().unwrap()
            // }
            // else if local.as_str() == "tileheight" {
            //     map.tile_height = value.as_str().parse::<u32>().unwrap();
            // }
            // let mess = format!("map Attribute: {:?} = {:?}", local.as_str(), value.as_str() );
            // LoggerWeb::log(&mess);
        }
    }
}

fn parse_layer(parser: &mut Tokenizer, map: &mut Array2D) {
    LoggerWeb::log("layer start");
    while let Some(result) = parser.next() {
        let token = result.unwrap();
        if is_start(&token, "data") {
            parse_data(parser, map);
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

fn parse_data(parser: &mut Tokenizer, map: &mut Array2D) {
    LoggerWeb::log("data start");
    while let Some(result) = parser.next() {
        let token = result.unwrap();
        if is_end(&token, "data") {
            LoggerWeb::log("layer end");
            return;
        }
        else if let Token::Text { text } = token {
            parse_data_text(map, text.as_str() );
        }
    }
}

fn parse_data_text(map: &mut Array2D, map_txt: &str) {
        let mut bytes = Vec::new();
        for x in map_txt.replace('\n', "").split(',') {
            let cell_num = x.parse::<u8>().unwrap();
            bytes.push(cell_num);
        };
    map.bytes = bytes
}
