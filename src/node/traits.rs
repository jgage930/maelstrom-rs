use anyhow::Result;

pub trait Node {
    type MessageType;

    fn respond(&mut self, input: Self::MessageType) -> Result<Self::MessageType>;
}
