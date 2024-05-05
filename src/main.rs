use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Write};
use csv::ReaderBuilder;
use rand::prelude::IteratorRandom;
use rand::seq::SliceRandom;
use geoutils::{Distance, Location};

#[derive(Debug, Clone)]
struct Airport {
    location: Location,
}

impl Airport {
    fn new(latitude: f64, longitude: f64) -> Self {
        let location = Location::new(latitude, longitude);
        Airport { location }
    }
}

#[derive(Debug, Clone)]
struct NodeWithDistanceAndPath {
    distance: f64,
    path: Vec<String>,
}

// Load data from airports.csv to construct location data
fn load_airports_from_csv(filename: &str) -> Result<HashMap<String, Airport>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut airports: HashMap<String, Airport> = HashMap::new();

    let mut csv_reader = ReaderBuilder::new().flexible(true).from_reader(reader);

    for result in csv_reader.records().skip(1) {
        let record = result?;
        if record.len() >= 9 {
            if let (Ok(latitude), Ok(longitude)) = (record.get(7).unwrap().parse::<f64>(), record.get(8).unwrap().parse::<f64>()) {
                let airport = Airport::new(latitude, longitude);
                airports.insert(record.get(5).unwrap().to_string(), airport);
            } else {
                eprintln!("Skipping line with invalid latitude or longitude: {:?}", record);
            }
        } else {
            eprintln!("Skipping line with invalid format: {:?}", record);
        }
    }

    Ok(airports)
}

// Load data from routes.csv to construct the adjacency list
fn load_adjacency_list_from_csv(
    filename: &str,
    airports: &HashMap<String, Airport>,
) -> Result<HashMap<String, Vec<(String, f64)>>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut adjacency_list: HashMap<String, Vec<(String, f64)>> = HashMap::new();

    let mut csv_reader = ReaderBuilder::new().flexible(true).from_reader(reader);

    for result in csv_reader.records().skip(1) {
        let record = result?;
        if record.len() >= 6 {
            let from = record.get(3).unwrap().to_string(); // Source airport
            let to = record.get(5).unwrap().to_string(); // Destination airport

            // Ensure both source and destination airports exist in the airports map
            if let (Some(from_airport), Some(to_airport)) = (airports.get(&from), airports.get(&to)) {
                let distance = from_airport.location.distance_to(&to_airport.location).unwrap(); // Distance in meters
                let distance_km = distance.meters() as f64 / 1000.0; // Convert to kilometers
                // Add source airport to destination's neighbor list
                adjacency_list.entry(from.clone()).or_insert_with(Vec::new).push((to.clone(), distance_km));
                // Add destination airport to source's neighbor list
                adjacency_list.entry(to.clone()).or_insert_with(Vec::new).push((from.clone(), distance_km));
            } else {
                eprintln!("Missing location data for airports in route: {:?} - {:?}", from, to);
            }
        } else {
            eprintln!("Skipping line with invalid format: {:?}", record);
        }
    }

    Ok(adjacency_list)
}

// BFS algorithm to calculate distances from a source node to all other nodes
fn bfs(adjacency_list: &HashMap<String, Vec<(String, f64)>>, source: &str) -> HashMap<String, NodeWithDistanceAndPath> {
    let mut distances: HashMap<String, NodeWithDistanceAndPath> = HashMap::new();
    let mut visited: HashSet<String> = HashSet::new();
    let mut queue: Vec<(String, f64, Vec<String>)> = Vec::new();

    visited.insert(source.to_string());
    queue.push((source.to_string(), 0.0, vec![source.to_string()]));

    while let Some((node, dist, path)) = queue.pop() {
        distances.insert(node.clone(), NodeWithDistanceAndPath { distance: dist, path: path.clone() });

        if let Some(neighbors) = adjacency_list.get(&node) {
            for (neighbor, distance) in neighbors {
                if !visited.contains(neighbor) {
                    visited.insert(neighbor.clone());
                    let mut new_path = path.clone();
                    new_path.push(neighbor.clone());
                    queue.push((neighbor.clone(), dist + distance, new_path));
                }
            }
        }
    }

    distances
}

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
