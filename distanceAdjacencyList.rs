use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use geoutils::Location;
use csv::ReaderBuilder;

#[derive(Debug, Clone)]
struct Airport {
    iata: String,
    location: Location,
}

impl Airport {
    fn new(iata: String, latitude: f64, longitude: f64) -> Self {
        let location = Location::new(latitude, longitude);
        Airport { iata, location }
    }
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
            let iata = record.get(5).unwrap().to_string(); // IATA code
            if let (Ok(latitude), Ok(longitude)) = (record.get(7).unwrap().parse::<f64>(), record.get(8).unwrap().parse::<f64>()) {
                airports.insert(iata.clone(), Airport::new(iata, latitude, longitude));
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
                let distance = from_airport.location.distance_to(&to_airport.location).unwrap().meters(); // Change to meters
                // Add source airport to destination's neighbor list
                adjacency_list.entry(from.clone()).or_insert_with(Vec::new).push((to.clone(), distance));
                // Add destination airport to source's neighbor list
                adjacency_list.entry(to.clone()).or_insert_with(Vec::new).push((from.clone(), distance));
            } else {
                eprintln!("Missing location data for airports in route: {:?} - {:?}", from, to);
            }
        } else {
            eprintln!("Skipping line with invalid format: {:?}", record);
        }
    }

    Ok(adjacency_list)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Load location data from airports.csv
    let airports = load_airports_from_csv("airports.csv")?;

    // Load adjacency list with connections from routes.csv using location data
    let adjacency_list = load_adjacency_list_from_csv("valid_routes.csv", &airports)?;

    // Print errors encountered during loading the adjacency list
    // and the first 100 entries of the adjacency list
    let mut count = 0;
    for (airport, neighbors) in adjacency_list.iter() {
        if count >= 50 {
            break;
        }
        print!("Airport {}: [", airport);
        for (neighbor, distance) in neighbors {
            println!("(\"{}\", {:.2} meters), ", neighbor, distance);
        }
        println!("]");
        count += 1;
    }

    Ok(())
}
