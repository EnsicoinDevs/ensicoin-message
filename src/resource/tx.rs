use ensicoin_serializer::types::Sha256Result;
use ensicoin_serializer::{Deserialize, Serialize};
use sha2::Digest;

use super::script::OP;
use crate::message::{Message, MessageType};

#[derive(
    Hash, Eq, PartialEq, Clone, Serialize, Deserialize, serde::Serialize, serde::Deserialize,
)]
pub struct Outpoint {
    pub hash: Sha256Result,
    pub index: u32,
}

#[derive(Hash, PartialEq, Eq, Serialize, Deserialize, serde::Serialize, serde::Deserialize)]
pub struct TransactionInput {
    pub previous_output: Outpoint,
    script: Vec<OP>,
}

#[derive(Hash, PartialEq, Eq, Serialize, Deserialize, serde::Serialize, serde::Deserialize)]
pub struct TransactionOutput {
    value: u64,
    script: Vec<OP>,
}

#[derive(Hash, PartialEq, Eq, Serialize, Deserialize, serde::Serialize, serde::Deserialize)]
pub struct Transaction {
    version: u32,
    flags: Vec<String>,
    inputs: Vec<TransactionInput>,
    outputs: Vec<TransactionOutput>,
}

impl Transaction {
    pub fn get_outputs(&self) -> &Vec<TransactionOutput> {
        &self.outputs
    }

    pub fn get_inputs(&self) -> &Vec<TransactionInput> {
        &self.inputs
    }
    pub fn double_hash(&self) -> Sha256Result {
        let bytes = self.serialize();
        let mut hasher = sha2::Sha256::default();
        hasher.input(bytes);
        hasher.result()
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
