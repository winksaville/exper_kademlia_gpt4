// kademlia/tests/find_node_test.rs

use std::collections::HashMap;
use std::ops::Range;
use std::{net::SocketAddr, sync::RwLock};
use std::sync::Arc;
use rand::Rng;
use rand::distributions::{Distribution, Uniform};

use tokio::runtime::Runtime;

use exper_kademlia_gpt4::{FindNodeTrait, Kademlia, Node };

use once_cell::sync::Lazy;

static TEST_NODES: Lazy<RwLock<HashMap<u64, Kademlia>>> = Lazy::new(|| RwLock::new(HashMap::new()));

fn add_node(knode: Kademlia) {
    let mut nodes = TEST_NODES.write().unwrap();
    let node_id = knode.node.id;
    if let Some(previous_node) = nodes.insert(node_id, knode) {
        println!("add_node({:?}): Already inserted, previous_node={:?}", node_id, previous_node);
    }
}

fn get_node(node_id: u64) -> Option<Node> {
    let nodes = TEST_NODES.read().unwrap();
    let result = if let Some(knode) = nodes.get(&node_id) {
        Some(knode.node.clone())
    } else {
        None
    };
    println!("get_node({}):+- result={result:?}", node_id);

    result
}

fn add_nodes(size: u64, ip_addr: &str, beg_port: u64, range_low: u64, range_high: u64, find_node_impl: Arc<dyn FindNodeTrait>) {
    let random_data = Uniform::new(range_low, range_high);
    let mut rand = rand::thread_rng();
    for i in 0..size {
        let node_id = random_data.sample(&mut rand);
        let addr: SocketAddr = format!("{}:{}", ip_addr, beg_port + i).parse().unwrap();
        let knode = Kademlia::new(Node { id: node_id, addr }, find_node_impl.clone());
        add_node(knode);
    }
}


#[derive(Clone, Debug)]
struct MockFindNode;

impl FindNodeTrait for MockFindNode {
    fn find_node_request(&self, _node: &Node, target: u64) -> Result<Vec<Node>, ()> {
        get_node(target)
            .map(|node| vec![node])
            .ok_or(())
    }
}

#[test]
fn test_node_not_found() {
    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    let node = Node { id: 1, addr };
    let find_node_impl = Arc::new(MockFindNode);

    let mut kademlia = Kademlia::new(node, find_node_impl);

    let target = 12345;
    let rt = Runtime::new().unwrap();
    let result = rt.block_on(kademlia.find_node(target));

    assert_eq!(result.len(), 0); // Since we return an empty Vec in MockFindNode, the result should be empty
}

#[test]
fn test_find_a_node() {

    add_nodes(10, "127.0.0.123", 8080, 0, 100, Arc::new(MockFindNode));
    println!("test_find_a_node: TEST_NODES={:?}", TEST_NODES);

    //let target_id = 12345;
    //let target_addr: SocketAddr = "127.0.0.1:8081".parse().unwrap();
    //add_node(Node { id: target_id, addr: target_addr });

    //let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    //let node = Node { id: 1, addr };
    //let find_node_impl = Arc::new(MockFindNode);

    //let mut kademlia = Kademlia::new(node, find_node_impl);

    //let rt = Runtime::new().unwrap();
    //let result = rt.block_on(kademlia.find_node(target_id));

    //assert_eq!(result.len(), 1);
    //assert_eq!(result[0].id, target_id);
}
