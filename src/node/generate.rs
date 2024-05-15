use super::traits::Node;
use crate::{Body, Message};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct GenerateNode;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum GeneratePayload {
    Generate,
    GenerateOk { id: String },
}

#[derive(Serialize, Deserialize)]
pub struct Generate {
    pub id: String,
}

pub type GenerateMessage = Message<Body<GeneratePayload>>;

impl Node for GenerateNode {
    type MessageType = GenerateMessage;

    fn respond(&mut self, input: Self::MessageType) -> anyhow::Result<Self::MessageType> {
        let id = Uuid::new_v4().to_string();
        let payload = GeneratePayload::GenerateOk { id };

        Ok(Message {
            src: input.dest,
            dest: input.src,
            body: Body {
                msg_id: Some(1),
                in_reply_to: input.body.msg_id,
                payload,
            },
        })
    }
}
