use crate::engine::AsnTexture;
use crate::map::AsnMap;
use crate::tileset::AsnTileSet;
use asn_core::math::Size3D;
use asn_logger::debug;

pub fn load_tiles(tsx_buf: &[u8]) -> AsnTileSet {
    let tiles = asn_decoder_tiled::load_tsx(tsx_buf).unwrap();
    println!("{:?}", tiles);
    let mut asn_tileset = AsnTileSet {
        tile_size: tiles.tile_size,
        tile_count: tiles.tile_count,
        columns: tiles.columns,
    };
    asn_tileset
}

pub fn load_map(tmx_buf: &[u8]) -> AsnMap {
    let map = asn_decoder_tiled::load_tmx(tmx_buf).unwrap();
    let map_size = Size3D {
        width: map.size.width,
        height: map.size.height,
        depth: map.layers.len() as u32,
    };
    let mut asn_map = AsnMap::new(&map_size);
    for (mut layer_index, layer) in map.layers.iter().enumerate() {
        // println!("layer ------");
        // println!("layer_index: {:?}", layer_index);
        // println!("{:?}", layer.name);
        // println!("{:?}", layer.id);
        // println!("{:?}", layer.size);
        // println!("{:?}", layer.visible);
        // println!("{:?}", layer.bytes.len());
        let src = layer.bytes.as_slice();
        // println!("{:?}", src.len());
        asn_map.copy_layer(layer_index, &layer.size, src);
        layer_index += 1;
    }
    asn_map
}
