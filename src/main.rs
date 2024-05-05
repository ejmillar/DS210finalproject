mod airports;
mod graph;
mod bfs;

use airports::load_airports_from_csv;
use graph::load_adjacency_list_from_csv;
use bfs::{bfs, NodeWithDistanceAndPath}; // Import NodeWithDistanceAndPath here

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use rand::prelude::IteratorRandom; // Add this import

fn main() -> Result<(), Box<dyn Error>> {
    // Load location data from airports.csv
    let airports = load_airports_from_csv("airports.csv")?;

    // Load adjacency list with connections from routes.csv using location data
    let adjacency_list = load_adjacency_list_from_csv("routes.csv", &airports)?;

    // Randomly sample x number of nodes for sampling
    let mut rng = rand::thread_rng();
    let mut sampled_nodes: Vec<_> = adjacency_list.keys().choose_multiple(&mut rng, 10);

    // Open output.txt to write results
    let mut output_file = File::create("output.txt")?;

    // Write the adjacency list to output.txt
    for (airport, neighbors) in &adjacency_list {
        writeln!(output_file, "Airport {}: {:?}", airport, neighbors)?;
    }

    let mut total_distance = 0.0;
    let mut pair_count = 0;

    // Calculate distances from the sampled nodes to all other airports
    for sampled_node in sampled_nodes.iter() {
        let distances = bfs(&adjacency_list, *sampled_node);

        // Write distances and paths from the sampled node to all other airports to output.txt
        for (airport, node_with_distance_path) in &distances {
            writeln!(output_file, "Distance from {} to {}: {:.2} kilometers", sampled_node, airport, node_with_distance_path.distance)?;
            writeln!(output_file, "Path: {:?}", node_with_distance_path.path)?;

            // Skip nodes with infinite distance
            if node_with_distance_path.distance != std::f64::INFINITY {
                // Add up the distances between each pair
                total_distance += node_with_distance_path.distance;
                pair_count += 1;
            }
        }
    }

    // Calculate the average distance
    let average_distance = if pair_count > 0 {
        total_distance / pair_count as f64
    } else {
        0.0
    };

    // Output the average distance
    writeln!(output_file, "\nAverage distance between every reachable airport within sampled pairs: {:.2} kilometers", average_distance)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn test_bfs() {
        // Sample dataset for testing
        let mut adjacency_list: HashMap<String, Vec<(String, f64)>> = HashMap::new();
        adjacency_list.insert("A".to_string(), vec![("B".to_string(), 1.0)]);
        adjacency_list.insert("B".to_string(), vec![("C".to_string(), 1.0)]);
        adjacency_list.insert("C".to_string(), vec![("D".to_string(), 1.0)]);

        // Run BFS from node "A"
        let distances = bfs(&adjacency_list, "A");

        // Expected distances and paths
        let expected_distances: HashMap<String, NodeWithDistanceAndPath> = [
            ("A".to_string(), NodeWithDistanceAndPath { distance: 0.0, path: vec!["A".to_string()] }),
            ("B".to_string(), NodeWithDistanceAndPath { distance: 1.0, path: vec!["A".to_string(), "B".to_string()] }),
            ("C".to_string(), NodeWithDistanceAndPath { distance: 2.0, path: vec!["A".to_string(), "B".to_string(), "C".to_string()] }),
            ("D".to_string(), NodeWithDistanceAndPath { distance: 3.0, path: vec!["A".to_string(), "B".to_string(), "C".to_string(), "D".to_string()] }),
        ].iter().cloned().collect();

        // Check if the actual distances match the expected ones
        for (node, expected_node) in expected_distances.iter() {
            assert_eq!(distances.get(node), Some(expected_node));
        }
    }

    #[test]
    fn test_bfs_unreachable_airports() {
        // Sample dataset for testing with unreachable airports
        let mut adjacency_list: HashMap<String, Vec<(String, f64)>> = HashMap::new();
        adjacency_list.insert("A".to_string(), vec![("B".to_string(), 1.0)]);
        adjacency_list.insert("B".to_string(), vec![("C".to_string(), 1.0)]);
        adjacency_list.insert("C".to_string(), vec![("D".to_string(), 1.0)]);
        adjacency_list.insert("E".to_string(), vec![("F".to_string(), 1.0)]);

        // Run BFS from node "A"
        let distances = bfs(&adjacency_list, "A");

        // Expected distances and paths
        let expected_distances: HashMap<String, NodeWithDistanceAndPath> = [
            ("A".to_string(), NodeWithDistanceAndPath { distance: 0.0, path: vec!["A".to_string()] }),
            ("B".to_string(), NodeWithDistanceAndPath { distance: 1.0, path: vec!["A".to_string(), "B".to_string()] }),
            ("C".to_string(), NodeWithDistanceAndPath { distance: 2.0, path: vec!["A".to_string(), "B".to_string(), "C".to_string()] }),
            ("D".to_string(), NodeWithDistanceAndPath { distance: 3.0, path: vec!["A".to_string(), "B".to_string(), "C".to_string(), "D".to_string()] }),
        ].iter().cloned().collect();

        // Check if the actual distances match the expected ones
        for (node, expected_node) in expected_distances.iter() {
            if let Some(actual_node) = distances.get(node) {
                // Check distance
                assert_eq!(actual_node.distance, expected_node.distance, "Distance mismatch for node {}", node);
                // Check path
                assert_eq!(actual_node.path, expected_node.path, "Path mismatch for node {}", node);
            } else {
                // Node not found in actual distances, assert that it's unreachable
                assert!(!adjacency_list.contains_key(node), "Node {} should not be reachable", node);
            }
        }
    }


}
