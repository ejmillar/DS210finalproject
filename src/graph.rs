use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::airports::Airport;
use geoutils::Location;

pub fn load_adjacency_list_from_csv(
    filename: &str,
    airports: &HashMap<String, Airport>,
) -> Result<HashMap<String, Vec<(String, f64)>>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut adjacency_list: HashMap<String, Vec<(String, f64)>> = HashMap::new();

    for line in reader.lines().skip(1) {
        let record = line?;
        let fields: Vec<_> = record.split(',').collect();
        if fields.len() >= 6 {
            let from = fields[3].to_string(); // Source airport
            let to = fields[5].to_string(); // Destination airport

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
