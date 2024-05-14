use super::traits::Node;
use crate::{Body, Message};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct BroadcastNode {
    /// Store the messages this node has seen.
    message_ids: Vec<usize>,
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
    pub toplogy: HashMap<String, Vec<String>>,
}

pub type BroadcastMessage = Message<Body<BroadcastPayload>>;

impl Node for BroadcastNode {
    type MessageType = BroadcastMessage;

    fn respond(&self, input: Self::MessageType) -> anyhow::Result<Self::MessageType> {
        let reply_payload = match input.body.payload {
            BroadcastPayload::Broadcast(b) => {
                self.message_ids.push(b.message);

                BroadcastPayload::BroadcastOk
            }
            BroadcastPayload::Read => todo!(),
            BroadcastPayload::Topology(t) => todo!(),
            _ => panic!("Cannot respond to message type."),
        };

        Ok(())
    }
}
