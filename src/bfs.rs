use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq)]
pub struct NodeWithDistanceAndPath {
    pub distance: f64,
    pub path: Vec<String>,
}

pub fn bfs(adjacency_list: &HashMap<String, Vec<(String, f64)>>, source: &str) -> HashMap<String, NodeWithDistanceAndPath> {
    let mut distances: HashMap<String, NodeWithDistanceAndPath> = HashMap::new();
    let mut visited: HashSet<String> = HashSet::new();
    let mut queue: Vec<(String, f64, Vec<String>)> = Vec::new();

    visited.insert(source.to_string());
    queue.push((source.to_string(), 0.0, vec![source.to_string()]));

    while let Some((node, dist, path)) = queue.pop() {
        // Insert the node's information into the distances HashMap
        distances.insert(node.clone(), NodeWithDistanceAndPath { distance: dist, path: path.clone() });

        if let Some(neighbors) = adjacency_list.get(&node) {
            for (neighbor, distance) in neighbors {
                if !visited.contains(neighbor) {
                    // Update the visited set and queue with the new neighbor
                    visited.insert(neighbor.clone());
                    let mut new_path = path.clone();
                    new_path.push(neighbor.clone());
                    queue.push((neighbor.clone(), dist + distance, new_path));
                }
            }
        }
    }

    // Mark unreachable nodes as infinite distance
    for node in adjacency_list.keys() {
        if !distances.contains_key(node) {
            distances.insert(node.clone(), NodeWithDistanceAndPath { distance: std::f64::INFINITY, path: vec![] });
        }
    }

    distances
}
