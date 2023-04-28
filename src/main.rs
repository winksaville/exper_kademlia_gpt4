//use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::net::SocketAddr;
use std::net::UdpSocket;
use serde::{Deserialize, Serialize};

const BUCKET_SIZE: usize = 20;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
struct Node {
    id: u64,
    addr: SocketAddr,
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

#[derive(Debug)]
#[allow(unused)]
struct Kademlia {
    node: Node,
    buckets: Vec<BinaryHeap<NodeDistance>>,
    socket: UdpSocket,
}

#[allow(unused)]
impl Kademlia {
    async fn find_node(&mut self, target: u64) -> Vec<Node> {
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
            let response = self.send_find_node_request(&current_node.node, target).await;

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

    async fn send_find_node_request(&self, _node: &Node, _target: u64) -> Result<Vec<Node>, ()> {
        // Send find_node request to the given node and return the list of nodes received
        // This function is a placeholder for the actual implementation of sending and receiving
        // messages over the network
        //unimplemented!()
        Ok(vec![])
    }
}

fn main() {
    // Example usage of Kademlia::find_node
}

