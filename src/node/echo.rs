use super::traits::Node;
use crate::{Body, Message};
use anyhow::Result;
use serde::{Deserialize, Serialize};

pub struct EchoNode;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum EchoPayload {
    Echo(Echo),
    EchoOk(Echo),
}

#[derive(Serialize, Deserialize)]
pub struct Echo {
    pub echo: String,
}

pub type EchoMessage = Message<Body<EchoPayload>>;

impl Node for EchoNode {
    type MessageType = EchoMessage;

    fn respond(&mut self, input: Self::MessageType) -> Result<Self::MessageType> {
        let echo = match &input.body.payload {
            EchoPayload::Echo(p) => &p.echo,
            EchoPayload::EchoOk(_) => {
                panic!("Cannot read from EchoOk");
            }
        };

        let payload = EchoPayload::EchoOk(Echo {
            echo: echo.to_string(),
        });

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

    fn from_init(init: super::init::InitMessage) -> Self {
        EchoNode
    }
}
