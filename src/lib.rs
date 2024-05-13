use serde::{Deserialize, Serialize};

/// Highest level wrapper for a message sent or recieved by a node
/// generic over any body B
pub struct Message<B> {
    src: String,
    dest: String,
    body: B,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Body<P> {
    pub msg_id: Option<usize>,
    pub in_reply_to: Option<usize>,

    #[serde(flatten)]
    pub payload: P,
}
