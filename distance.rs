use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use geoutils::Location;

fn main() -> Result<(), Box<dyn Error>> {
    // Read coordinates from the CSV file
    let mut locations = Vec::new();
    let mut iata_to_index = HashMap::new();
    let file = File::open("airports.csv")?;
    let reader = BufReader::new(file);
    let mut csv_reader = csv::Reader::from_reader(reader);
    for result in csv_reader.records() {
        let record = result?;
        let iata = record[5].to_string();
        let latitude: f64 = record[7].parse()?;
        let longitude: f64 = record[8].parse()?;
        locations.push(Location::new(latitude, longitude));
        iata_to_index.insert(iata.clone(), locations.len() - 1);
    }

    // Generate random pairs of airport IATA codes
    let mut rng = rand::thread_rng();
    let iata_codes: Vec<&String> = iata_to_index.keys().collect();
    let random_pairs: Vec<(&String, &String)> = (0..100)
        .map(|_| {
            let iata1 = *iata_codes.choose(&mut rng).unwrap();
            let iata2 = *iata_codes.choose(&mut rng).unwrap();
            (iata1, iata2)
        })
        .collect();

    // Calculate distances for random pairs of airports
    for (iata1, iata2) in random_pairs {
        let idx1 = iata_to_index[iata1];
        let idx2 = iata_to_index[iata2];
        let location1 = locations[idx1];
        let location2 = locations[idx2];
        let distance = location1.distance_to(&location2).unwrap();

        let distance_in_km = distance.meters() / 1000.0;
        println!(
            "Distance between Airport {} and Airport {} = {:.2} kilometers",
            iata1, iata2, distance_in_km
        );
    }

    Ok(())
}
