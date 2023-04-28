// kademlia/src/lib.rs

//use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::fmt::Debug;

//use tokio::net::UdpSocket;

use serde::{Deserialize, Serialize};

pub const BUCKET_SIZE: usize = 20;

// The FindNodeTrait
//pub trait FindNodeTrait: Send + Sync + Debug {
//  async fn find_node_request(&self, node: &Node, target: u64) -> Result<Vec<Node>, ()>;
//}
pub trait FindNodeTrait {
    fn find_node_request(&self, node: &Node, target: u64) -> Result<Vec<Node>, ()>;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Node {
    pub id: u64,
    pub addr: SocketAddr,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
struct NodeDistance {
    node: Node,
    distance: u64,
}

impl NodeDistance {
    fn new(target: u64, node: Node) -> Self {
        let distance = target ^ node.id;
        NodeDistance { node, distance }
    }
}

//#[derive(Debug)]
pub struct Kademlia {
    #[allow(unused)]
    node: Node,
    buckets: Vec<BinaryHeap<NodeDistance>>,
    find_node_impl: Arc<dyn FindNodeTrait>,
}

impl Kademlia {
    pub fn new(node: Node, find_node_impl: Arc<dyn FindNodeTrait>) -> Self {
        Kademlia {
            node,
            buckets: vec![BinaryHeap::new(); 160],
            find_node_impl,
        }
    }

    pub async fn find_node(&mut self, target: u64) -> Vec<Node> {
        let mut closest_nodes = BinaryHeap::with_capacity(BUCKET_SIZE);
        let mut visited_nodes = Vec::new();

        for bucket in &self.buckets {
            for node_distance in bucket.iter() {
                if closest_nodes.len() < BUCKET_SIZE {
                    closest_nodes.push(node_distance.clone());
                } else {
                    let max_distance = closest_nodes.peek().unwrap().distance;
                    if node_distance.distance < max_distance {
                        closest_nodes.pop();
                        closest_nodes.push(node_distance.clone());
                    }
                }
            }
        }

        while let Some(current_node) = closest_nodes.pop() {
            visited_nodes.push(current_node.node.clone());
            let response = self.find_node_impl.find_node_request(&current_node.node, target);

            match response {
                Ok(nodes) => {
                    for node in nodes {
                        if visited_nodes.contains(&node) {
                            continue;
                        }

                        let node_distance = NodeDistance::new(target, node);
                        if closest_nodes.len() < BUCKET_SIZE {
                            closest_nodes.push(node_distance);
                        } else {
                            let max_distance = closest_nodes.peek().unwrap().distance;
                            if node_distance.distance < max_distance {
                                closest_nodes.pop();
                                closest_nodes.push(node_distance);
                            }
                        }
                    }
                }
                Err(_) => continue,
            }
        }

        visited_nodes
    }
}
