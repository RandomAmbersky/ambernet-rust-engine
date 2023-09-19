use crate::map::AsnMap;
use asn_tiled::load_tmx;

pub fn load_map(tmx_buf: &[u8]) -> AsnMap {
    let map = asn_tiled::load_tmx(tmx_buf).unwrap();
    let asn_map = AsnMap::new(&map.size);
    asn_map
}
