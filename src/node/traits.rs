use anyhow::Result;

pub trait Node {
    type MessageType;

    fn respond(&self, input: Self::MessageType) -> Result<Self::MessageType>;
}
