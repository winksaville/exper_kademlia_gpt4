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
pub trait FindNodeTrait: Send + Sync + Debug {
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

#[derive(Clone, Debug)]
pub struct Kademlia {
    #[allow(unused)]
    pub node: Node,
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

    pub fn update_bucket(&mut self, node: Node) {
        let target = self.node.id;
        let node_distance = NodeDistance::new(target, node);
        let bucket_index = node_distance.distance.leading_zeros() as usize;
        if let Some(bucket) = self.buckets.get_mut(bucket_index) {
            if let Some(entry) = bucket.iter_mut().find(|entry| entry.node.id == node.id)
        }
    }

    pub fn join(&mut self, bootstrap_node: Node) -> Result<(), ()> {
        let target = self.node.id;

        // Perform an initial find_node request on the bootstrap node
        let closest_nodes = self.find_node_impl.find_node_request(&bootstrap_node, target);

        // Update the bucket with the bootstrap node
        self.update_bucket(bootstrap_node);

        // Update the bucket with the nodes found in the initial find_node request
        for node in closest_nodes {
            self.update_bucket(node);
        }

        Ok(())
    }

    pub async fn find_node(&mut self, target: u64) -> Vec<Node> {
        println!("find_node({}):+", target);
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
        println!("find_node({}): closest_nodes.len={}", target, closest_nodes.len());

        while let Some(current_node) = closest_nodes.pop() {
            println!("find_node({}): TOL visiting current_node={:?}", target, current_node);
            visited_nodes.push(current_node.node.clone());
            let response = self.find_node_impl.find_node_request(&current_node.node, target);
            println!("find_node({}): visiting current_node={:?} response={:?}", target, current_node, response);

            match response {
                Ok(nodes) => {
                    for node in nodes {
                        if visited_nodes.contains(&node) {
                            continue;
                        }

                        let node_distance = NodeDistance::new(target, node);
                        if closest_nodes.len() < BUCKET_SIZE {
                            println!("find_node({}): visiting current_node={:?} node_distance{:?} bucket not full has now closest_nodes.len={}", target, current_node, node_distance, closest_nodes.len() + 1);
                            closest_nodes.push(node_distance);
                        } else {
                            let max_distance = closest_nodes.peek().unwrap().distance;
                            if node_distance.distance < max_distance {
                                println!("find_node({}): visiting current_node={:?} new node is closer, node_distance={:?} < max_distance={}", target, current_node, node_distance, max_distance);
                                closest_nodes.pop();
                                closest_nodes.push(node_distance);
                            }
                        }
                    }
                }
                Err(_) => continue,
            }
        }

        println!("find_node({}):- visited_nodes.len={}", target, visited_nodes.len());
        visited_nodes
    }
}
