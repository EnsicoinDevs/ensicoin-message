use ensicoin_serializer::{Deserialize, Serialize};

use crate::message::{Message, MessageType};

#[derive(Serialize, Deserialize, Debug)]
pub struct GetMempool;

impl Message for GetMempool {
    fn message_string() -> [u8; 12] {
        [103, 101, 116, 109, 101, 109, 112, 111, 111, 108, 0, 0]
    }
    fn message_type() -> MessageType {
        MessageType::GetMempool
    }
}
