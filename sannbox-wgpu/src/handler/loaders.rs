use crate::map::AsnMap;
use asn_core::math::Size3D;

pub fn load_map(tmx_buf: &[u8]) -> AsnMap {
    let map = asn_tiled::load_tmx(tmx_buf).unwrap();
    let map_size = Size3D {
        width: map.size.width,
        height: map.size.height,
        depth: map.layers.len() as u32,
    };
    let asn_map = AsnMap::new(&map_size);
    for layer in map.layers.iter() {
        println!("{:?}", layer.name);
        println!("{:?}", layer.id);
        println!("{:?}", layer.size);
        println!("{:?}", layer.visible);
        layer.bytes.len();
    }
    asn_map
}
