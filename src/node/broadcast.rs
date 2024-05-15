use super::traits::Node;
use crate::{Body, Message};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

pub struct BroadcastNode {
    /// Store the messages this node has seen.
    pub message_ids: Vec<usize>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum BroadcastPayload {
    Broadcast { message: usize },
    BroadcastOk,
    Read,
    ReadOk { messages: Vec<usize> },
    Topology { topology: HashMap<String, String> },
    TopologyOk,
}

pub type BroadcastMessage = Message<Body<BroadcastPayload>>;

impl Node for BroadcastNode {
    type MessageType = BroadcastMessage;

    fn respond(&mut self, input: Self::MessageType) -> anyhow::Result<Self::MessageType> {
        let reply_payload = match input.body.payload {
            BroadcastPayload::Broadcast { message } => {
                self.message_ids.push(message);

                BroadcastPayload::BroadcastOk
            }
            BroadcastPayload::Read => BroadcastPayload::ReadOk {
                messages: self.message_ids.clone(),
            },
            BroadcastPayload::Topology { topology } => BroadcastPayload::TopologyOk,
            _ => panic!("Cannot respond to message type."),
        };

        Ok(Message {
            src: input.dest,
            dest: input.src,
            body: Body {
                msg_id: Some(1),
                in_reply_to: input.body.msg_id,
                payload: reply_payload,
            },
        })
    }
}
