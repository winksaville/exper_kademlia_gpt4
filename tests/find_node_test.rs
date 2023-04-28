// kademlia/tests/find_node_test.rs

use std::net::SocketAddr;
use std::sync::Arc;

use tokio::runtime::Runtime;

use exper_kademlia_gpt4::{FindNodeTrait, Kademlia, Node };

#[derive(Clone)]
struct MockFindNode;

impl FindNodeTrait for MockFindNode {
    fn find_node_request(&self, _node: &Node, _target: u64) -> Result<Vec<Node>, ()> {
        // Return an empty Vec to simulate no nodes found for testing purposes
        Ok(Vec::new())
    }
}

#[test]
fn test_find_node() {
    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    let node = Node { id: 1, addr };
    let find_node_impl = Arc::new(MockFindNode);

    let mut kademlia = Kademlia::new(node, find_node_impl);

    let target = 12345;
    let rt = Runtime::new().unwrap();
    let result = rt.block_on(kademlia.find_node(target));

    assert_eq!(result.len(), 0); // Since we return an empty Vec in MockFindNode, the result should be empty
}
