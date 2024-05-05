use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufRead};
use geoutils::{Distance, Location};

#[derive(Debug, Clone)]
pub struct Airport {
    pub location: Location,
}

impl Airport {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        let location = Location::new(latitude, longitude);
        Airport { location }
    }
}

pub fn load_airports_from_csv(filename: &str) -> Result<HashMap<String, Airport>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut airports: HashMap<String, Airport> = HashMap::new();

    for line in reader.lines().skip(1) {
        let record = line?;
        let fields: Vec<_> = record.split(',').collect();
        if fields.len() >= 9 {
            if let (Ok(latitude), Ok(longitude)) = (fields[7].parse::<f64>(), fields[8].parse::<f64>()) {
                let airport = Airport::new(latitude, longitude);
                airports.insert(fields[5].to_string(), airport);
            } else {
                eprintln!("Skipping line with invalid latitude or longitude: {:?}", record);
            }
        } else {
            eprintln!("Skipping line with invalid format: {:?}", record);
        }
    }

    Ok(airports)
}
