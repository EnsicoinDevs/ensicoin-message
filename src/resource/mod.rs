pub mod block;
pub mod script;
pub mod tx;

pub use block::{Block, BlockHeader};
pub use tx::{Outpoint, Transaction};

#[cfg(feature = "grpc")]
mod grpc {
    #[derive(Debug)]
    pub enum ConvertError {
        MissingField(&'static str),
        InvalidHashSize(usize),
        DeserializationError(ensicoin_serializer::Error),
    }
    impl From<ensicoin_serializer::Error> for ConvertError {
        fn from(err: ensicoin_serializer::Error) -> ConvertError {
            Self::DeserializationError(err)
        }
    }
    include!(concat!(env!("OUT_DIR"), "/ensicoin_rpc.rs"));
}
