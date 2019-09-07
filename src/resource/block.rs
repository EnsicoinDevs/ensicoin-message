use ensicoin_serializer::{hash_to_string, types::Sha256Result, Deserialize, Serialize};

use sha2::Digest;

use crate::{
    message::{Message, MessageType},
    resource::Transaction,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct BlockHeader {
    pub version: u32,
    pub flags: Vec<String>,
    pub prev_block: Sha256Result,
    pub merkle_root: Sha256Result,
    pub timestamp: u64,
    pub height: u32,
    pub target: Sha256Result,
    pub nonce: u64,
}

impl std::fmt::Debug for BlockHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BlockHeader")
            .field("version", &self.version)
            .field("flags", &self.flags)
            .field("prev_block", &hash_to_string(&self.prev_block))
            .field("merkle_root", &hash_to_string(&self.merkle_root))
            .field("timestamp", &self.timestamp)
            .field("height", &self.height)
            .field("target", &hash_to_string(&self.target))
            .field("nonce", &self.nonce)
            .finish()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub txs: Vec<Transaction>,
}

#[cfg(feature = "grpc")]
impl std::convert::TryFrom<super::grpc::Block> for Block {
    type Error = super::grpc::ConvertError;
    fn try_from(value: super::grpc::Block) -> Result<Self, Self::Error> {
        if value.prev_block.len() != 32 {
            return Err(Self::Error::InvalidHashSize(value.prev_block.len()));
        }
        if value.merkle_root.len() != 32 {
            return Err(Self::Error::InvalidHashSize(value.merkle_root.len()));
        }
        if value.target.len() != 32 {
            return Err(Self::Error::InvalidHashSize(value.target.len()));
        }
        let mut txs = Vec::with_capacity(value.txs.len());
        for tx in value.txs {
            txs.push(Transaction::try_from(tx)?);
        }
        let header = BlockHeader {
            version: value.version,
            flags: value.flags,
            prev_block: Sha256Result::clone_from_slice(&value.prev_block),
            merkle_root: Sha256Result::clone_from_slice(&value.merkle_root),
            timestamp: value.timestamp,
            height: value.height,
            target: Sha256Result::clone_from_slice(&value.target),
            nonce: value.nonce,
        };
        Ok(Self { header, txs })
    }
}

impl BlockHeader {
    pub fn double_hash(&self) -> Sha256Result {
        let bytes = self.serialize();
        let mut hasher = sha2::Sha256::default();
        hasher.input(bytes);
        let first = hasher.result();
        hasher = sha2::Sha256::default();
        hasher.input(first);
        hasher.result()
    }
}

impl Block {
    pub fn double_hash(&self) -> Sha256Result {
        self.header.double_hash()
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
