use super::{init::InitPayload, traits::Node};
use crate::{Body, Message};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

pub struct BroadcastNode {
    id: String,
    /// A vec of ids of all nodes in cluster,
    node_ids: Vec<String>,
    /// The neighbors that this node can gossip to, comes from topology
    neighbors: Vec<String>,
    /// Store the messages this node has seen.
    pub message_ids: HashSet<usize>,
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
                self.message_ids.insert(message);

                BroadcastPayload::BroadcastOk
            }
            BroadcastPayload::Read => BroadcastPayload::ReadOk {
                messages: self.message_ids.clone().into_iter().collect::<Vec<usize>>(),
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

    fn from_init(init: super::init::InitPayload) -> Result<Self> {
        match init {
            InitPayload::InitOk => panic!("Cannot create a node from InitOk."),
            InitPayload::Init { node_id, node_ids } => {
                let node = BroadcastNode {
                    id: node_id,
                    node_ids,
                    neighbors: Vec::new(),
                    message_ids: HashSet::new(),
                };

                Ok(node)
            }
        }
    }
}
