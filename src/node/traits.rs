use anyhow::Result;

pub trait Node {
    type MessageType;

    fn respond(input: Self::MessageType) -> Result<Self::MessageType>;
}
