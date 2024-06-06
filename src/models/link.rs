use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Link {
    pub id: String,
    pub target_url: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkTarget {
    pub target_url: String,
}
