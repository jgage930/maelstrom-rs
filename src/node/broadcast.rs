use super::traits::Node;
use crate::{Body, Message};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct BroadcastNode {
    /// Store the messages this node has seen.
    pub message_ids: Vec<usize>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum BroadcastPayload {
    Broadcast(Broadcast),
    BroadcastOk,
    Read,
    ReadOk(ReadOk),
    Topology(Topology),
    TopologyOk,
}

#[derive(Serialize, Deserialize)]
pub struct Broadcast {
    pub message: usize,
}

#[derive(Serialize, Deserialize)]
pub struct ReadOk {
    pub messages: Vec<usize>,
}

#[derive(Serialize, Deserialize)]
pub struct Topology {
    pub topology: HashMap<String, Vec<String>>,
}

pub type BroadcastMessage = Message<Body<BroadcastPayload>>;

impl Node for BroadcastNode {
    type MessageType = BroadcastMessage;

    fn respond(&mut self, input: Self::MessageType) -> anyhow::Result<Self::MessageType> {
        let reply_payload = match input.body.payload {
            BroadcastPayload::Broadcast(b) => {
                self.message_ids.push(b.message);

                BroadcastPayload::BroadcastOk
            }
            BroadcastPayload::Read => BroadcastPayload::ReadOk(ReadOk {
                messages: self.message_ids.clone(),
            }),
            BroadcastPayload::Topology(_t) => BroadcastPayload::TopologyOk,
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
