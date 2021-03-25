use serde::{Deserialize, Serialize//};
use std::net::SocketAddr;

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct NodeRef {
    pub address: SocketAddr,
}

impl NodeRef {
    pub fn new(address: SocketAddr) -> NodeRef {
        NodeRef { address }
    }

    pub  fn get(&self, key: &str) -> Option<String> {
        dbg!(key);
        None
    }

    pub  fn upsert(&self, key: String, value: String) {
        dbg!(key, value);
    }

    pub  fn upsert_prefix_ref(&self, prefix: &str, addr: SocketAddr) {
        dbg!(prefix, addr);
    }
}
