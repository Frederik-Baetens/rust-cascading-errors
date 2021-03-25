mod noderef;
use noderef::NodeRef;

fn main() {
    let addr = "0.0.0.0:8000".parse().unwrap();
    let noderef = NodeRef::new(addr);
    noderef.get("hi");
    noderef.upsert("hi".to_owned(), "hey".to_owned());
    noderef.upsert_prefix_ref("hi", addr);
}
