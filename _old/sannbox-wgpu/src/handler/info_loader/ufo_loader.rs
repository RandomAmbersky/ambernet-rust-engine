use crate::handler::resources::ALIEN_TXT;
use serde::{Deserialize, Serialize};

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

#[allow(dead_code)]
pub fn load_ufo() {
    let alien_config: AlienInfo = serde_yaml::from_slice(ALIEN_TXT).unwrap();
    let str = serde_yaml::to_string(&alien_config).unwrap();
    panic!("alien_str: {:?}", alien_config);
}
