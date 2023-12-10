mod data_loader;
mod lang_loader;
mod ufo_loader;

pub use data_loader::{load_data, DataSet};
pub use lang_loader::{load_lang, LangSet};

// use crate::handler::resources::INFO_TXT;
// use serde::{Deserialize, Serialize};
// use std::collections::HashMap;
//
// #[derive(Debug, Serialize, Deserialize)]
// struct EntityInfo {
//     name: String,
//     info: String,
// }
//
// #[derive(Debug, Serialize, Deserialize)]
// struct Info {
//     entity_list: HashMap<String, EntityInfo>,
// }

// pub fn load_info() {
//     // let mut entity_list: HashMap<String, EntityInfo> = HashMap::new();
//     // entity_list.insert(
//     //     "GreenContainer".to_string(),
//     //     EntityInfo {
//     //         name: "Зеленый контейнер".to_string(),
//     //         info: "Контейнер с биомассой. Судя по цвету - растительного происхождения".to_string(),
//     //     },
//     // );
//     //
//     // let base_config: Info = Info { entity_list };
//     // panic!(
//     //     "base_config: {}",
//     //     serde_yaml::to_string(&base_config).unwrap()
//     // );
//     // let alien_config: AlienInfo = serde_yaml::from_str(&str).unwrap();
//     // println!("scrape_config: {:?}", scrape_config);
//     // let tex_config: AlienInfo = serde_yaml::from_str(&str).unwrap();
// }
