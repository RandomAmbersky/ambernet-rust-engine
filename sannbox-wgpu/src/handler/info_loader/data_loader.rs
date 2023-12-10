use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct EntityInfo {
    pub types: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataSet {
    pub entity_list: HashMap<String, EntityInfo>,
}

pub fn load_data(buf: &[u8]) -> DataSet {
    let res: DataSet = serde_yaml::from_slice(buf).unwrap();
    res
}
