use ensicoin_serializer::{types::Sha256Result, Deserialize, Serialize};
use sha2::Digest;
#[cfg(feature = "grpc")]
use std::convert::TryFrom;

use super::script::OP;
use crate::message::{Message, MessageType};

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct Outpoint {
    pub hash: Sha256Result,
    pub index: u32,
}

#[cfg(feature = "grpc")]
impl TryFrom<super::grpc::Outpoint> for Outpoint {
    type Error = super::grpc::ConvertError;

    fn try_from(value: super::grpc::Outpoint) -> Result<Self, Self::Error> {
        if value.hash.len() != 32 {
            return Err(Self::Error::InvalidHashSize(value.hash.len()));
        }
        Ok(Self {
            hash: Sha256Result::clone_from_slice(&value.hash),
            index: value.index,
        })
    }
}

#[derive(Hash, PartialEq, Eq, Serialize, Deserialize, Clone, Debug)]
pub struct TransactionInput {
    pub previous_output: Outpoint,
    pub script: Vec<OP>,
}

#[cfg(feature = "grpc")]
impl TryFrom<super::grpc::TxInput> for TransactionInput {
    type Error = super::grpc::ConvertError;

    fn try_from(value: super::grpc::TxInput) -> Result<Self, Self::Error> {
        let previous_output = match value.previous_output {
            Some(o) => Outpoint::try_from(o)?,
            None => return Err(Self::Error::MissingField("previous_output")),
        };
        let mut de = ensicoin_serializer::Deserializer::new(bytes::BytesMut::from(value.script));
        Ok(Self {
            previous_output,
            script: Vec::deserialize(&mut de)?,
        })
    }
}

#[derive(Hash, PartialEq, Eq, Serialize, Deserialize, Clone, Debug)]
pub struct TransactionOutput {
    pub value: u64,
    pub script: Vec<OP>,
}

#[cfg(feature = "grpc")]
impl TryFrom<super::grpc::TxOutput> for TransactionOutput {
    type Error = super::grpc::ConvertError;

    fn try_from(value: super::grpc::TxOutput) -> Result<Self, Self::Error> {
        let mut de = ensicoin_serializer::Deserializer::new(bytes::BytesMut::from(value.script));
        Ok(Self {
            value: value.value,
            script: Vec::deserialize(&mut de)?,
        })
    }
}

#[derive(Hash, PartialEq, Eq, Serialize, Deserialize, Clone, Debug)]
pub struct Transaction {
    pub version: u32,
    pub flags: Vec<String>,
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<TransactionOutput>,
}

#[cfg(feature = "grpc")]
impl TryFrom<super::grpc::Tx> for Transaction {
    type Error = super::grpc::ConvertError;

    fn try_from(value: super::grpc::Tx) -> Result<Self, Self::Error> {
        let mut inputs = Vec::with_capacity(value.inputs.len());
        for input in value.inputs {
            inputs.push(TransactionInput::try_from(input)?);
        }
        let mut outputs = Vec::with_capacity(value.outputs.len());
        for output in value.outputs {
            outputs.push(TransactionOutput::try_from(output)?);
        }
        Ok(Self {
            version: value.version,
            flags: value.flags,
            inputs,
            outputs,
        })
    }
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
}

impl Message for Transaction {
    fn message_string() -> [u8; 12] {
        [116, 120, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    }
    fn message_type() -> MessageType {
        MessageType::Transaction
    }
}
