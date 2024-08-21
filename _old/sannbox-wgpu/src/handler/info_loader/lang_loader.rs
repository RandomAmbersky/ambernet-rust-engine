use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct LangSetData {
    pub name: String,
    pub info: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LangSet {
    pub entity_list: HashMap<String, LangSetData>,
}

#[allow(dead_code)]
pub fn load_lang(buf: &[u8]) -> LangSet {
    let res: LangSet = serde_yaml::from_slice(buf).unwrap();
    res
}
