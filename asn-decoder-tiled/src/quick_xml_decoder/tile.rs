use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Tile {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@type")]
    _type: String,
}
