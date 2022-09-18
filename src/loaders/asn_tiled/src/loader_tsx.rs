use xmlparser::{Token, Tokenizer};
use amberskynet_logger_web::LoggerWeb;
use asn_core::{Size2D};
use crate::utils::{is_end, is_start};

#[derive(Default, Debug)]
pub struct DecodedTileset {
    name: String,
    source: String,
    layer_size: Size2D,
    tile_size: Size2D,
    tile_count: u32,
    columns: u32,
}

pub fn parse(text: &str) -> Result<DecodedTileset, String> {
    let mut tileset = DecodedTileset::default();

    let mess = format!("parsed map is: {:?}", text);
    LoggerWeb::log(&mess);

    let mut parser = Tokenizer::from(text);
    while let Some(result) = parser.next() {
        let token = result.unwrap();
        if is_start(&token, "tileset") {
            parse_tileset(&mut parser, &mut tileset)
        }
    }

    Ok(tileset)
}

fn parse_tileset(parser: &mut Tokenizer, tileset: &mut DecodedTileset) {
    LoggerWeb::log("tileset start");
    while let Some(result) = parser.next() {
        let token = result.unwrap();
        if let Token::Attribute { local, value, .. } = token {
            if local.as_str() == "name" {
                tileset.name = value.to_string();
            }
            if local.as_str() == "tilewidth" {
                tileset.tile_size.width = value.to_string().parse::<u32>().unwrap();
            }
            if local.as_str() == "tileheight" {
                tileset.tile_size.height = value.to_string().parse::<u32>().unwrap();
            }
            if local.as_str() == "tilecount" {
                tileset.tile_count = value.to_string().parse::<u32>().unwrap();
            }
            if local.as_str() == "columns" {
                tileset.columns = value.to_string().parse::<u32>().unwrap();
            }
            if local.as_str() == "source" {
                tileset.source = value.to_string();
            }
            if local.as_str() == "width" {
                tileset.layer_size.width = value.to_string().parse::<u32>().unwrap();
            }
            if local.as_str() == "height" {
                tileset.layer_size.height = value.to_string().parse::<u32>().unwrap();
            }
            let mess = format!("layer Attribute: {:?} = {:?}", local.as_str(), value.as_str() );
            LoggerWeb::log(&mess);
        }
    }
}
