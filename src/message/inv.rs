use ensicoin_serializer::types::Sha256Result;
use ensicoin_serializer::{Deserialize, Serialize};

use crate::message::{Message, MessageType};

#[derive(Serialize, Deserialize)]
pub struct InvVect {
    pub data_type: crate::message::ResourceType,
    pub hash: Sha256Result,
}

impl std::fmt::Debug for InvVect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use ensicoin_serializer::hash_to_string;
        f.debug_struct("InvVect")
            .field("data_type", &self.data_type)
            .field("hash", &hash_to_string(&self.hash))
            .finish()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Inv {
    pub inventory: Vec<InvVect>,
}

impl Message for Inv {
    fn message_string() -> [u8; 12] {
        [105, 110, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    }
    fn message_type() -> MessageType {
        MessageType::Inv
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetData {
    pub inventory: Vec<InvVect>,
}

impl Message for GetData {
    fn message_string() -> [u8; 12] {
        [103, 101, 116, 100, 97, 116, 97, 0, 0, 0, 0, 0]
    }
    fn message_type() -> MessageType {
        MessageType::GetData
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NotFound {
    pub inventory: Vec<InvVect>,
}

impl Message for NotFound {
    fn message_string() -> [u8; 12] {
        [110, 111, 116, 102, 111, 117, 110, 100, 0, 0, 0, 0]
    }
    fn message_type() -> MessageType {
        MessageType::NotFound
    }
}
