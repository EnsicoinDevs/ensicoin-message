use ensicoin_serializer::types::Sha256Result;
use ensicoin_serializer::{Deserialize, Serialize};

use sha2::Digest;

use crate::message::{Message, MessageType};
use crate::resource::Transaction;

#[derive(Serialize, Deserialize, Clone)]
pub struct BlockHeader {
    pub version: u32,
    pub flags: Vec<String>,
    pub prev_block: Sha256Result,
    pub merkle_root: Sha256Result,
    pub timestamp: u64,
    pub height: u32,
    pub bits: u32,
    pub nonce: u64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Block {
    pub header: BlockHeader,
    pub txs: Vec<Transaction>,
}

impl Block {
    pub fn double_hash(&self) -> Sha256Result {
        let bytes = self.serialize();
        let mut hasher = sha2::Sha256::default();
        hasher.input(bytes);
        hasher.result()
    }
}

impl Message for Block {
    fn message_string() -> [u8; 12] {
        [98, 108, 111, 99, 107, 0, 0, 0, 0, 0, 0, 0]
    }

    fn message_type() -> MessageType {
        MessageType::Block
    }
}
