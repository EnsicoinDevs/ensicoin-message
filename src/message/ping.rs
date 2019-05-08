use super::{Message, MessageType};
use ensicoin_serializer::Serialize;

#[derive(Serialize)]
pub struct Ping {}

impl Message for Ping {
    fn message_string() -> [u8; 12] {
        [50, 112, 108, 117, 115, 50, 105, 115, 52, 0, 0, 0]
    }
    fn message_type() -> MessageType {
        MessageType::Ping
    }
}

impl Ping {
    pub fn new() -> Ping {
        Ping {}
    }
}

#[derive(Serialize)]
pub struct Pong {}

impl Message for Pong {
    fn message_string() -> [u8; 12] {
        [109, 105, 110, 117, 115, 49, 116, 104, 97, 116, 115, 51]
    }
    fn message_type() -> MessageType {
        MessageType::Pong
    }
}

impl Pong {
    pub fn new() -> Pong {
        Pong {}
    }
}
