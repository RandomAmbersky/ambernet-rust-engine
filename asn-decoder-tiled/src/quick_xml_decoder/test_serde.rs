#[cfg(test)]
mod tests {
    use crate::quick_xml_decoder::{from_xml, to_xml};

    const MAP_TSX: &[u8] = include_bytes!("../tiles.tsx");

    #[test]
    fn load_serde_tsx_ok() {
        let map_str = match std::str::from_utf8(MAP_TSX) {
            Ok(v) => v.replace('\n', "").replace("> <", "><"),
            Err(err) => panic!("Error: {:?}", err.to_string()),
        };

        let item = from_xml(map_str.as_str());
        let str = to_xml(item);

        assert_eq!(map_str, str);
    }
}
