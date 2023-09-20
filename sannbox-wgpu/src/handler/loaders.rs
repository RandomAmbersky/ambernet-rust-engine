use crate::map::AsnMap;

pub fn load_map(tmx_buf: &[u8]) -> AsnMap {
    let map = asn_tiled::load_tmx(tmx_buf).unwrap();
    let asn_map = AsnMap::new(&map.size);
    for layer in map.layers.iter() {
        println!("{:?}", layer.name);
        println!("{:?}", layer.id);
        println!("{:?}", layer.size);
        println!("{:?}", layer.visible);
        layer.bytes
    }
    asn_map
}
