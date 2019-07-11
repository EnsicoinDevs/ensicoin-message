use crate::message::{Message, MessageType};
use bytes::Bytes;
use ensicoin_serializer::{Deserialize, Deserializer, Result, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct GetAddr;

impl Message for GetAddr {
    fn message_string() -> [u8; 12] {
        [103, 101, 116, 97, 100, 100, 114, 0, 0, 0, 0, 0]
    }
    fn message_type() -> MessageType {
        MessageType::GetAddr
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Address {
    pub timestamp: u64,
    pub ip: [u8; 16],
    pub port: u16,
}

impl Deserialize for Address {
    fn deserialize(de: &mut Deserializer) -> Result<Self> {
        let timestamp = u64::deserialize(de)?;
        let ip_bytes = de.extract_bytes(16)?;
        let mut ip = [0; 16];
        for (i, b) in ip_bytes.iter().enumerate() {
            ip[i] = *b;
        }
        let port = u16::deserialize(de)?;

        Ok(Address {
            timestamp,
            ip,
            port,
        })
    }
}

impl Serialize for Address {
    fn serialize(&self) -> Bytes {
        let mut buf = Bytes::new();
        buf.extend_from_slice(&self.timestamp.serialize());
        buf.extend_from_slice(&self.ip);
        buf.extend_from_slice(&self.port.serialize());
        buf
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Addr {
    pub addresses: Vec<Address>,
}

impl Message for Addr {
    fn message_string() -> [u8; 12] {
        [97, 100, 100, 114, 0, 0, 0, 0, 0, 0, 0, 0]
    }
    fn message_type() -> MessageType {
        MessageType::Addr
    }
}
