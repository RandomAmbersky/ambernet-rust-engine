use xmlparser::{Token, Tokenizer};
#[allow(unused_imports)]
use amberskynet_logger_web::LoggerWeb;
use crate::decoded_map::{DecodedTileInfo, DecodedTileset};
use crate::DecodedMap;
use crate::utils::{is_end, is_start};

// pub fn parse(text: &str) -> Result<DecodedMap, String> {
//     let mut tileset = DecodedTileset::default();
//
//     // let mess = format!("parsed map is: {:?}", text);
//     // LoggerWeb::log(&mess);
//
//     let mut parser = Tokenizer::from(text);
//     while let Some(result) = parser.next() {
//         let token = result.unwrap();
//         if is_start(&token, "tileset") {
//             parse_tileset(&mut parser, &mut tileset)
//         }
//     }
//
//     Ok(tileset)
// }

pub fn parse_tileset(parser: &mut Tokenizer, map: &mut DecodedMap) {
    // LoggerWeb::log("tileset start");
    let mut tileset = DecodedTileset::default();
    while let Some(result) = parser.next() {
        let token = result.unwrap();
        if is_end(&token, "tileset") {
            map.tilesets.push(tileset);
            // LoggerWeb::log("tileset end");
            return;
        }
        if is_start(&token, "image") {
            parse_image(parser, &mut tileset);
        }
        if is_start(&token, "tile") {
            parse_tile(parser, &mut tileset);
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
