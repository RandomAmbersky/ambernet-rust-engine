use xmlparser::{Token, Tokenizer};
use amberskynet_logger_web::LoggerWeb;
use asn_core::{Array2D, Size2D};
use crate::utils::{is_end, is_start};

#[derive(Default, Debug)]
pub struct DecodedLayer {
    pub name: String,
    pub id: u32,
    pub size: Size2D,
    pub bytes: Vec<u8>,
    pub visible: bool
}

#[derive(Default, Debug)]
pub struct DecodedMap {
    pub size: Size2D,
    pub layers: Vec<DecodedLayer>
}

pub fn parse(text: &str) -> Result<DecodedMap, String> {
    let mut map = DecodedMap::default();

    let mut parser = Tokenizer::from(text);
    while let Some(result) = parser.next() {
        let token = result.unwrap();
        if is_start(&token, "map") {
            parse_map(&mut parser, &mut map);
        }
    }

    Ok(map)
}

fn parse_map(parser: &mut Tokenizer, map: &mut DecodedMap) {
    // LoggerWeb::log("map start");
    while let Some(result) = parser.next() {
        let token = result.unwrap();
        if is_end(&token, "map") {
            // LoggerWeb::log("map end");
            return;
        }
        if is_start(&token, "layer") {
            parse_layer(parser, map);
        } else if let Token::Attribute { local, value,  .. } = token {
            if local.as_str() == "width" {
                map.size.width = value.as_str().parse::<u32>().unwrap();
            }
            else if local.as_str() == "height" {
                map.size.height = value.as_str().parse::<u32>().unwrap();
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

fn parse_layer(parser: &mut Tokenizer, map: &mut DecodedMap) {
    let mut layer = DecodedLayer::default();
    // LoggerWeb::log("layer start");
    while let Some(result) = parser.next() {
        let token = result.unwrap();
        if is_start(&token, "data") {
            parse_data(parser, &mut layer);
        }
        else if is_end(&token, "layer") {
            // LoggerWeb::log("layer end");
            map.layers.push(layer);
            return;
        }
        else if let Token::Attribute { local, value,  .. } = token {
            if local.as_str() == "id" {
                layer.id = value.to_string().parse::<u32>().unwrap();
            }
            if local.as_str() == "name" {
                layer.name = value.to_string();
            }
            // 	let mess = format!("layer Attribute: {:?} = {:?}", local.as_str(), value.as_str() );
        // 	LoggerWeb::log(&mess);
        }
    }
}

fn parse_data(parser: &mut Tokenizer, layer: &mut DecodedLayer) {
    // LoggerWeb::log("data start");
    while let Some(result) = parser.next() {
        let token = result.unwrap();
        if is_end(&token, "data") {
            // LoggerWeb::log("data end");
            return;
        }
        else if let Token::Text { text } = token {
            parse_data_text(layer, text.as_str() );
        }
    }
}

fn parse_data_text(layer: &mut DecodedLayer, map_txt: &str) {
        let mut bytes = Vec::new();
        for x in map_txt.replace('\n', "").split(',') {
            let cell_num = x.parse::<u8>().unwrap();
            bytes.push(cell_num);
        };
    layer.bytes = bytes
}
