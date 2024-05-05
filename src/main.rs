mod airports;
mod graph;
mod bfs;

use airports::load_airports_from_csv;
use graph::load_adjacency_list_from_csv;
use bfs::bfs;

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

    // Randomly sample 3 nodes for sampling
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

            // Add up the distances between each pair
            total_distance += node_with_distance_path.distance;
            pair_count += 1;
        }
    }

    // Calculate the average distance
    let average_distance = if pair_count > 0 {
        total_distance / pair_count as f64
    } else {
        0.0
    };

    // Output the average distance
    writeln!(output_file, "\nAverage distance between sampled pairs: {:.2} kilometers", average_distance)?;

    Ok(())
}
