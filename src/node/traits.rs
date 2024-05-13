pub trait Node {
    type MessageType;

    fn respond(input: &Self::MessageType) -> Self::MessageType;
}
