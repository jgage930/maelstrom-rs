use maelstrom_rs::{Body, Message};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum InitPayload {
    Init(Init),
    InitOk,
}

#[derive(Serialize, Deserialize)]
pub struct Init {
    pub node_id: String,
    pub node_ids: Vec<String>,
}

fn init_node(init_message: Message<Body<InitPayload>>) -> Message<Body<InitPayload>> {}
