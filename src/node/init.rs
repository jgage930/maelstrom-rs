use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum InitPayload {
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk,
}
