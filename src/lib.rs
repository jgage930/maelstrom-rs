pub mod node;

use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};
use serde_json;
use std::io::Write;

/// Highest level wrapper for a message sent or recieved by a node
/// generic over any body B
#[derive(Serialize, Deserialize)]
pub struct Message<B> {
    src: String,
    dest: String,
    body: B,
}

impl<B> Message<B> {
    pub fn reply(&self, output: &mut impl Write) -> Result<()>
    where
        B: Serialize,
    {
        serde_json::to_writer(&mut *output, self)?;
        output.write_all(b"\n")?;

        Ok(())
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Body<P> {
    pub msg_id: Option<usize>,
    pub in_reply_to: Option<usize>,

    #[serde(flatten)]
    pub payload: P,
}
