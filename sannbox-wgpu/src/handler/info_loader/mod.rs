use crate::handler::resources::ALIEN_TXT;
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AlienItemSet {
    // item_sets: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AlienDeploymentData {
    alien_rank: u32,
    item_sets: Vec<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AlienDeployment {
    #[serde(rename = "type")]
    _type: String,
    enviro_effects: Option<String>,
    data: Vec<AlienDeploymentData>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AlienInfo {
    alien_deployments: Vec<AlienDeployment>,
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
    let alien_config: AlienInfo = serde_yaml::from_slice(ALIEN_TXT).unwrap();
    let str = serde_yaml::to_string(&alien_config).unwrap();
    panic!("alien_str: {:?}", alien_config);
    // let alien_config: AlienInfo = serde_yaml::from_str(&str).unwrap();

    panic!(
        "alien_config: {}",
        serde_yaml::to_string(&alien_config).unwrap()
    );

    // println!("scrape_config: {:?}", scrape_config);
}
