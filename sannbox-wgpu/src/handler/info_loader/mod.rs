use crate::handler::resources::INFO_TXT;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct EntityInfo {
    name: String,
    info: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Info {
    entity_list: HashMap<String, EntityInfo>,
}

pub fn load_info() {
    // let mut entity_list: HashMap<String, EntityInfo> = HashMap::new();
    // entity_list.insert(
    //     "GreenContainer".to_string(),
    //     EntityInfo {
    //         name: "Зеленый контейнер".to_string(),
    //         info: "Контейнер с биомассой. Судя по цвету - растительного происхождения".to_string(),
    //     },
    // );

    // let base_config: Info = Info { entity_list };
    // panic!(
    //     "base_config: {}",
    //     serde_yaml::to_string(&base_config).unwrap()
    // );
    let scrape_config: Info = serde_yaml::from_slice(INFO_TXT).unwrap();
    // panic!(
    //     "base_config: {}",
    //     serde_yaml::to_string(&scrape_config).unwrap()
    // );

    // println!("scrape_config: {:?}", scrape_config);
}
