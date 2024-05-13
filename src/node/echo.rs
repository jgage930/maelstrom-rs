use core::panic;

use super::traits::Node;
use crate::{Body, Message};
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

type EchoMessage = Message<Body<EchoPayload>>;

impl Node for EchoNode {
    type MessageType = EchoMessage;

    fn respond(input: &Self::MessageType) -> Self::MessageType {
        let echo = match &input.body.payload {
            EchoPayload::Echo(p) => &p.echo,
            EchoPayload::EchoOk(_) => {
                panic!("Cannot read from EchoOk");
            }
        };

        let payload = EchoPayload::EchoOk(Echo {
            echo: echo.to_string(),
        });

        Message {
            src: input.dest.clone(),
            dest: input.src.clone(),
            body: Body {
                msg_id: Some(1),
                in_reply_to: input.body.msg_id,
                payload,
            },
        }
    }
}
