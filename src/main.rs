use std::{collections::HashMap, fs};

#[derive(Debug)]
pub struct CountryData {
    min: f64,
    max: f64,
    mean: f64,
    count: u128,
    sum: f64,
}

// pub struct Output {
//     data: HashMap<String, CountryData>,
// }

type Output = HashMap<String, CountryData>;

fn main() {
    let file_path: String = String::from("./measurements.txt");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut output: Output = HashMap::new();

    // TODO
    // have to delete this
    let mut count = 0;
    for line in contents.lines() {
        let parts: Vec<&str> = line.split(";").collect();

        let country: String = parts[0].to_string();
        let measurement: f64 = parts[1].parse().expect("Failed to parse to f32");

        if let Some(country_data) = output.get_mut(&country) {
            country_data.min = country_data.min.min(measurement);
            country_data.max = country_data.max.max(measurement);
            country_data.sum = country_data.sum + measurement;
            country_data.count = country_data.count + 1;
        } else {
            output.insert(
                country,
                CountryData {
                    min: measurement,
                    max: measurement,
                    mean: 0.0,
                    sum: measurement,
                    count: 1,
                },
            );
        }
        count = count + 1;
        println!("{}", count);
    }

    for (country, country_data) in &mut output {
        country_data.mean = country_data.sum / country_data.count as f64;
    }

    // Convert the HashMap into a Vec of tuples and sort it by key
    let mut sorted_output: Vec<(String, CountryData)> = output.into_iter().collect();
    sorted_output.sort_by_key(|k| k.0.clone());

    if let Some(tuple) = sorted_output.first() {
        println!("{}", tuple.0);
        println!("{:?}", tuple.1);
    }
}
