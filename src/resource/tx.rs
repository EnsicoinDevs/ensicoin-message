use ensicoin_serializer::types::Sha256Result;
use ensicoin_serializer::{Deserialize, Serialize};
use sha2::Digest;

use super::script::{Script, OP};
use crate::message::{Message, MessageType};

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct Outpoint {
    pub hash: Sha256Result,
    pub index: u32,
}

#[derive(Hash, PartialEq, Eq, Serialize, Deserialize, Clone, Debug)]
pub struct TransactionInput {
    pub previous_output: Outpoint,
    pub script: Vec<OP>,
}

#[derive(Hash, PartialEq, Eq, Serialize, Deserialize, Clone, Debug)]
pub struct TransactionOutput {
    pub value: u64,
    pub script: Vec<OP>,
}

#[derive(Hash, PartialEq, Eq, Serialize, Deserialize, Clone, Debug)]
pub struct Transaction {
    pub version: u32,
    pub flags: Vec<String>,
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<TransactionOutput>,
}

impl Transaction {
    pub fn double_hash(&self) -> Sha256Result {
        let bytes = self.serialize();
        let mut hasher = sha2::Sha256::default();
        hasher.input(bytes);
        let first = hasher.result();
        hasher = sha2::Sha256::default();
        hasher.input(first);
        hasher.result()
    }

    pub fn shash(
        &self,
        i: usize,
        referenced_outpoint: &Outpoint,
        referenced_script: &Script,
    ) -> Sha256Result {
        let mut hasher_outpoints = sha2::Sha256::default();
        for input in &self.inputs {
            hasher_outpoints.input(input.previous_output.serialize());
        }
        let outpoint_simple_hash = hasher_outpoints.result();
        let mut hasher = sha2::Sha256::default();
        hasher.input(outpoint_simple_hash);
        let outpoint_hash = hasher.result();

        let mut hasher = sha2::Sha256::default();
        hasher.input(self.version.serialize());
        hasher.input(self.flags.serialize());
        hasher.input(outpoint_hash);
    }
}

impl Message for Transaction {
    fn message_string() -> [u8; 12] {
        [116, 120, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    }
    fn message_type() -> MessageType {
        MessageType::Transaction
    }
}
