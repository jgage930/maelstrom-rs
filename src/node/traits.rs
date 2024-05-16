use super::init::InitPayload;
use anyhow::Result;

pub trait Node {
    type MessageType;

    fn respond(&mut self, input: Self::MessageType) -> Result<Self::MessageType>;
    fn from_init(init: InitPayload) -> Result<Self>
    where
        Self: Sized;
}
