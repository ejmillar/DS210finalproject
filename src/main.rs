extern crate rand;

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

// Define a function to load data from a CSV file and construct the adjacency list
fn load_adjacency_list_from_csv(filename: &str) -> Result<HashMap<String, Vec<String>>, std::io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut adjacency_list: HashMap<String, Vec<String>> = HashMap::new();

    for line in reader.lines().skip(1) {
        let line = line?;
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() >= 6 {
            let from = parts[3].to_string(); // Source airport
            let to = parts[5].to_string(); // Destination airport

            // Add source airport to destination's neighbor list
            adjacency_list.entry(from.clone()).or_insert_with(Vec::new).push(to.clone());

            // Add destination airport to source's neighbor list
            adjacency_list.entry(to.clone()).or_insert_with(Vec::new).push(from);
        }
    }

    Ok(adjacency_list)
}

// BFS algorithm to calculate distances from a source node to all other nodes
fn bfs(adjacency_list: &HashMap<String, Vec<String>>, source: &str) -> HashMap<String, i32> {
    let mut distances: HashMap<String, i32> = HashMap::new();
    let mut visited: HashSet<String> = HashSet::new();
    let mut queue: Vec<(String, i32)> = Vec::new();

    visited.insert(source.to_string());
    queue.push((source.to_string(), 0));

    while let Some((node, dist)) = queue.pop() {
        distances.insert(node.clone(), dist);
        if let Some(neighbors) = adjacency_list.get(&node) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    visited.insert(neighbor.clone());
                    queue.push((neighbor.clone(), dist + 1));
                }
            }
        }
    }

    distances
}

// Function to print the adjacency list for the first 100 airports
fn print_adjacency_list(adjacency_list: &HashMap<String, Vec<String>>, num_airports: usize) {
    let mut count = 0;
    for (airport, neighbors) in adjacency_list {
        println!("Airport {}: {:?}", airport, neighbors);
        count += 1;
        if count >= num_airports {
            break;
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    // Load data from routes.csv and construct the adjacency list
    let adjacency_list = load_adjacency_list_from_csv("routes.csv")?;

    // Print the adjacency list for the first 100 airports
    print_adjacency_list(&adjacency_list, 100);

    // Randomly select nodes for sampling
    let mut rng = thread_rng();
    let mut sampled_nodes = adjacency_list.keys().collect::<Vec<_>>();
    sampled_nodes.shuffle(&mut rng);
    let sampled_nodes = sampled_nodes.iter().take(10).map(|&s| s.to_string()).collect::<Vec<_>>();

    // Calculate distances between sampled nodes
    let mut total_distance = 0;
    let mut num_pairs = 0;

    for source_node in &sampled_nodes {
        let distances = bfs(&adjacency_list, source_node);
        for (_, &distance) in &distances {
            // Increment the total distance for each pair of nodes
            total_distance += distance;
            num_pairs += 1;
        }
    }

    // Calculate average distance
    let average_distance = total_distance as f64 / num_pairs as f64;
    println!("Average distance between all pairs of airports: {:.2}", average_distance);

    Ok(())
}
