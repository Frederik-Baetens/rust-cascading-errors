use serde::{Deserialize, Serialize//};
use std::net::SocketAddr;

fn main() {
    let addr = "0.0.0.0:8000".parse().unwrap();
    let noderef = NodeRef::new(addr);
    noderef.get("hi");
    noderef.upsert("hi".to_owned(), "hey".to_owned());
    noderef.upsert_prefix_ref("hi", addr);
}

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

