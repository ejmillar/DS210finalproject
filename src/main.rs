extern crate rand;

use rand::prelude::IteratorRandom; // Bring IteratorRandom trait into scope
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

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

fn main() -> Result<(), std::io::Error> {
    // Load data from routes.csv and construct the adjacency list
    let adjacency_list = load_adjacency_list_from_csv("routes.csv")?;

    // Randomly select a single node for sampling
    let mut rng = rand::thread_rng();
    let sampled_node = adjacency_list.keys().choose(&mut rng).unwrap().clone();

    // Open output.txt to write results
    let mut output_file = File::create("output.txt")?;

    // Write the adjacency list to output.txt
    for (airport, neighbors) in &adjacency_list {
        writeln!(output_file, "Airport {}: {:?}", airport, neighbors)?;
    }

    // Calculate distances from the sampled node to all other airports
    let distances = bfs(&adjacency_list, &sampled_node);

    // Write distances from the sampled node to all other airports to output.txt
    for (airport, &distance) in &distances {
        writeln!(output_file, "{}[{}: {}]", sampled_node, airport, distance)?;
    }

    // Calculate average distance (excluding distance to itself)
    let total_distance: i32 = distances.values().sum();
    let num_pairs = distances.len() - 1; // Subtract 1 to exclude distance to itself
    let average_distance = total_distance as f64 / num_pairs as f64;

    // Append the average distance to output.txt
    writeln!(output_file, "Average distance between all pairs of airports (excluding self): {:.2}", average_distance)?;

    Ok(())
}
