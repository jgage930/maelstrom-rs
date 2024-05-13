use maelstrom_rs::node::init::{Init, InitPayload};
use maelstrom_rs::{Body, Message};

/// A simple Echo server.  Instantiate a single echo node and echo back a message.
fn main() {
    let init_message = Message {
        src: "c1",
        dest: "n1",
        body: Body {
            msg_id: Some(1),
            in_reply_to: None,
            payload: InitPayload::Init(Init {
                node_id: "n1",
                node_ids: vec!["n1"],
            }),
        },
    };
}
