use std::collections::HashMap;
use std::fs::*;
use std::hash::Hash;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread::*;
use std::time::Instant;

#[derive(Debug)]
struct CityStats {
    min: f32,
    max: f32,
    count: f32,
    sum: f32,
}

type CityDataHashMap = HashMap<String, CityStats>;

fn get_min(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}

fn get_max(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}

fn process_data(data: &[u8]) -> HashMap<String, CityStats> {
    // Iterate over each split segment and print it
    let mut map: HashMap<String, CityStats> = HashMap::new();

    for segment in data.split(|&byte| byte == 10) {
        let mut parts = std::str::from_utf8(segment).unwrap().split(";");

        if let (Some(city), Some(value)) = (parts.next(), parts.next()) {
            let val = value.parse::<f32>().unwrap();

            match map.entry(city.to_string()) {
                std::collections::hash_map::Entry::Occupied(mut e) => {
                    let mut hash_map_value = e.get_mut();
                    hash_map_value.count += 1.0;
                    hash_map_value.sum += val;
                    hash_map_value.min = get_min(hash_map_value.min, val);
                    hash_map_value.max = get_max(hash_map_value.max, val);
                }
                std::collections::hash_map::Entry::Vacant(e) => {
                    e.insert(CityStats {
                        min: val,
                        max: val,
                        count: 0.0,
                        sum: val,
                    });
                }
            }
        }
    }

    map
}

fn main() {
    let d = Instant::now();
    let file_path = "./measurements.txt".to_string();
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // println!("Time taken for file upload: {:?}", d.elapsed());
    // let contents = "hello\nworld\nhello world\nhello world\nhello world\nhello world\nhello\nworld\nhello world\nhello world\nhello world\nhello world\n".to_string();

    let available_threads = available_parallelism().unwrap();
    println!("Available threads: {:?}", available_threads);

    let file_bytes = contents.as_bytes();

    let file_bytes_size = file_bytes.len();

    let min_slice_size = file_bytes_size / available_threads;

    let mut start_index = 0;
    let mut end_index = min_slice_size;
    let mut count = 0;

    let mut data_vec: Vec<&[u8]> = Vec::new();
    loop {
        while end_index < file_bytes_size && file_bytes[end_index] != 10 {
            end_index += 1;
        }
        count = count + 1;

        if end_index >= file_bytes_size {
            data_vec.push(&file_bytes[start_index..]);
            break;
        }

        data_vec.push(&file_bytes[start_index..end_index]);

        start_index = end_index + 1;
        end_index = start_index + min_slice_size;
    }

    let time_for_threads = Instant::now();

    let mut city_data_hash_map = Arc::new(Mutex::new(CityDataHashMap::new()));

    scope(|s| {
        for data in data_vec {
            let mut city_data_hash_map_clone = city_data_hash_map.clone();

            s.spawn(move || {
                let temp_map: HashMap<String, CityStats> = process_data(data);

                let mut main_hash_map = city_data_hash_map_clone.lock().unwrap();

                for (city, stats) in temp_map {
                    match main_hash_map.entry(city) {
                        std::collections::hash_map::Entry::Occupied(mut e) => {
                            let mut hash_map_value = e.get_mut();
                            hash_map_value.count += stats.count;
                            hash_map_value.sum += stats.sum;
                            hash_map_value.min = get_min(hash_map_value.min, stats.min);
                            hash_map_value.max = get_max(hash_map_value.max, stats.max);
                        }
                        std::collections::hash_map::Entry::Vacant(e) => {
                            e.insert(stats);
                        }
                    }
                }
            });
        }
    });

    println!(
        "time taken for all the threads {:?}",
        time_for_threads.elapsed()
    )
}
