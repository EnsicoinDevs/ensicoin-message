use bytes::Bytes;
use ensicoin_serializer::{Deserialize, Deserializer, Serialize};

#[derive(Hash, Clone, PartialEq, Eq, Debug)]
pub enum OP {
    False,
    True,
    Dup,
    Equal,
    Verify,
    Hash160,
    Checksig,
    Push(u8),
    Byte(u8),
}

impl Serialize for OP {
    fn serialize(&self) -> Bytes {
        let op_code: u8 = match self {
            OP::False => 0,
            OP::True => 80,
            OP::Push(n) | OP::Byte(n) => n.clone(),
            OP::Dup => 100,
            OP::Equal => 120,
            OP::Verify => 140,
            OP::Hash160 => 160,
            OP::Checksig => 170,
        };
        Bytes::from(vec![op_code])
    }
}

impl Deserialize for OP {
    fn deserialize(de: &mut Deserializer) -> ensicoin_serializer::Result<OP> {
        let code = match u8::deserialize(de)? {
            0 => OP::False,
            80 => OP::True,
            n if n <= 75 => OP::Push(n),
            100 => OP::Dup,
            120 => OP::Equal,
            140 => OP::Verify,
            160 => OP::Hash160,
            170 => OP::Checksig,
            n => OP::Byte(n),
        };
        Ok(code)
    }
}
