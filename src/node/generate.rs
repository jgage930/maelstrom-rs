use super::{echo::EchoMessage, traits::Node};
use crate::{Body, Message};
use serde::{Deserialize, Serialize};

pub struct GenerateNode;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum GeneratePayload {
    Generate,
    GenerateOk(Generate),
}

#[derive(Serialize, Deserialize)]
pub struct Generate {
    pub id: String,
}

pub type GenerateMessage = Message<Body<GeneratePayload>>;

impl Node for GenerateNode {
    type MessageType = EchoMessage;

    fn respond(input: Self::MessageType) -> anyhow::Result<Self::MessageType> {
        todo!()
    }
}
