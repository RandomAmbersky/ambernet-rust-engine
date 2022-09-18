use xmlparser::{Token, Tokenizer};
use amberskynet_logger_web::LoggerWeb;
use asn_core::{Size2D};
use crate::utils::{is_end, is_start};

#[derive(Default, Debug)]
pub struct DecodedTileInfo {
    id: u32,
    class: String
}

#[derive(Default, Debug)]
pub struct DecodedImageInfo {
    pub source: String,
    pub size: Size2D,
}

#[derive(Default, Debug)]
pub struct DecodedTileset {
    pub name: String,
    pub tile_size: Size2D,
    pub tile_count: u32,
    pub columns: u32,
    pub image_info: DecodedImageInfo,
    pub tiles: Vec<DecodedTileInfo>
}

pub fn parse(text: &str) -> Result<DecodedTileset, String> {
    let mut tileset = DecodedTileset::default();

    // let mess = format!("parsed map is: {:?}", text);
    // LoggerWeb::log(&mess);

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
    // LoggerWeb::log("tileset start");
    while let Some(result) = parser.next() {
        let token = result.unwrap();
        if is_end(&token, "tileset") {
            // LoggerWeb::log("tileset end");
            return;
        }
        if is_start(&token, "image") {
            parse_image(parser, tileset);
        }
        if is_start(&token, "tile") {
            parse_tile(parser, tileset);
        }
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
        }
    }
}


fn parse_image(parser: &mut Tokenizer, tileset: &mut DecodedTileset) {
    // LoggerWeb::log("image start");
    while let Some(result) = parser.next() {
        let token = result.unwrap();

        if let Token::ElementEnd { .. } = token {
            // LoggerWeb::log("image end");
            return;
        }
        if let Token::Attribute { local, value, .. } = token {
            if local.as_str() == "source" {
                tileset.image_info.source = value.to_string();
            }
            if local.as_str() == "width" {
                tileset.image_info.size.width = value.to_string().parse::<u32>().unwrap();
            }
            if local.as_str() == "height" {
                tileset.image_info.size.height = value.to_string().parse::<u32>().unwrap();
            }
        }

    }
}

fn parse_tile(parser: &mut Tokenizer, tileset: &mut DecodedTileset) {
    // LoggerWeb::log("tile start");
    let mut new_tile_info = DecodedTileInfo::default();
    while let Some(result) = parser.next() {
        let token = result.unwrap();
        if let Token::ElementEnd { .. } = token {
            // LoggerWeb::log("tile end");
            tileset.tiles.push(new_tile_info);
            return;
        }
        if let Token::Attribute { local, value, .. } = token {
            if local.as_str() == "id" {
                new_tile_info.id = value.to_string().parse::<u32>().unwrap();
            }
            if local.as_str() == "class" {
                new_tile_info.class = value.to_string();
            }
        }
    }
}
