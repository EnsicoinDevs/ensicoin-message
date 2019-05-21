use bytes::Bytes;
use ensicoin_serializer::{Deserialize, Deserializer, Serialize};

mod getblocks;
mod getmempool;
mod inv;
mod ping;
mod whoami;

pub use getblocks::GetBlocks;
pub use getmempool::GetMempool;
pub use inv::{GetData, Inv, InvVect, NotFound};
pub use ping::{Ping, Pong};
pub use whoami::{Whoami, WhoamiAck};

#[derive(Debug)]
pub enum ResourceType {
    Transaction,
    Block,
}

impl Serialize for ResourceType {
    fn serialize(&self) -> Bytes {
        match self {
            ResourceType::Block => (1 as u32).serialize(),
            ResourceType::Transaction => (0 as u32).serialize(),
        }
    }
}

impl Deserialize for ResourceType {
    fn deserialize(
        de: &mut ensicoin_serializer::Deserializer,
    ) -> ensicoin_serializer::Result<ResourceType> {
        match u32::deserialize(de) {
            Ok(0) => Ok(ResourceType::Transaction),
            Ok(1) => Ok(ResourceType::Block),
            Ok(n) => Err(ensicoin_serializer::Error::Message(format!(
                "Invalid DataType: {}",
                n
            ))),
            Err(e) => Err(ensicoin_serializer::Error::Message(format!(
                "Error reading DataType: {}",
                e
            ))),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum MessageType {
    Whoami,
    WhoamiAck,
    Inv,
    GetData,
    NotFound,
    GetBlocks,
    GetMempool,
    Block,
    Transaction,
    Ping,
    Pong,
    Unknown(Vec<u8>),
}

impl Deserialize for MessageType {
    fn deserialize(de: &mut Deserializer) -> ensicoin_serializer::Result<MessageType> {
        let raw_type = match de.extract_bytes(12) {
            Err(e) => {
                return Err(ensicoin_serializer::Error::Message(format!(
                    "Error in reading message type: {}",
                    e
                )));
            }
            Ok(v) => v.to_vec(),
        };
        Ok(
            if raw_type == [119, 104, 111, 97, 109, 105, 0, 0, 0, 0, 0, 0] {
                MessageType::Whoami
            } else if raw_type == [119, 104, 111, 97, 109, 105, 97, 99, 107, 0, 0, 0] {
                MessageType::WhoamiAck
            } else if raw_type == [50, 112, 108, 117, 115, 50, 105, 115, 52, 0, 0, 0] {
                MessageType::Ping
            } else if raw_type == [109, 105, 110, 117, 115, 49, 116, 104, 97, 116, 115, 51] {
                MessageType::Pong
            } else if raw_type == [105, 110, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0] {
                MessageType::Inv
            } else if raw_type == [103, 101, 116, 100, 97, 116, 97, 0, 0, 0, 0, 0] {
                MessageType::GetData
            } else if raw_type == [110, 111, 116, 102, 111, 117, 110, 100, 0, 0, 0, 0] {
                MessageType::NotFound
            } else if raw_type == [103, 101, 116, 98, 108, 111, 99, 107, 115, 0, 0, 0] {
                MessageType::GetBlocks
            } else if raw_type == [116, 120, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] {
                MessageType::Transaction
            } else if raw_type == [98, 108, 111, 99, 107, 0, 0, 0, 0, 0, 0, 0] {
                MessageType::Block
            } else if raw_type == [103, 101, 116, 109, 101, 109, 112, 111, 111, 108, 0, 0] {
                MessageType::GetMempool
            } else {
                MessageType::Unknown(raw_type)
            },
        )
    }
}

impl std::fmt::Display for MessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MessageType::Ping => "2plus2is4".to_string(),
                MessageType::Pong => "minus1thats3".to_string(),
                MessageType::Whoami => "Whoami".to_string(),
                MessageType::WhoamiAck => "WhoamiAck".to_string(),
                MessageType::Inv => "Inv".to_string(),
                MessageType::GetData => "GetData".to_string(),
                MessageType::NotFound => "NotFound".to_string(),
                MessageType::GetBlocks => "GetBlocks".to_string(),
                MessageType::GetMempool => "GetMempool".to_string(),
                MessageType::Transaction => "Transaction".to_string(),
                MessageType::Block => "Block".to_string(),
                MessageType::Unknown(s) => format!(
                    "Unknown: {}",
                    String::from_utf8(s.clone()).unwrap_or("<INVALID UTF8>".to_string())
                )
                .trim_matches('\x00')
                .to_string(),
            }
        )
    }
}

pub trait Message: Serialize {
    fn message_string() -> [u8; 12];
    fn message_type() -> MessageType;
    fn raw_bytes(&self) -> (MessageType, Bytes) {
        let magic: u32 = 422021;
        let message_string = Self::message_string();
        let payload = self.serialize();
        let payload_length: u64 = payload.len() as u64;

        let mut v = Bytes::new();
        v.extend_from_slice(&magic.serialize());
        v.extend_from_slice(&message_string);
        v.extend_from_slice(&payload_length.serialize());
        v.extend_from_slice(&payload);
        (Self::message_type(), v)
    }
}
