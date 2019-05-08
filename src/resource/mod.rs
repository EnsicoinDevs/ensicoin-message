use std::net::{IpAddr, SocketAddr};

use ensicoin_serializer::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

pub mod block;
pub mod script;
pub mod tx;

pub use block::Block;
pub use block::BlockHeader;
pub use tx::Outpoint;
pub use tx::Transaction;

#[derive(Serialize, Deserialize, serde::Serialize, serde::Deserialize)]
pub struct Address {
    pub timestamp: u64,
    pub address: SocketAddr,
}

impl Address {
    pub fn new() -> Address {
        Address {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            address: SocketAddr::new(IpAddr::from([127, 0, 0, 1]), 4224),
        }
    }
}
