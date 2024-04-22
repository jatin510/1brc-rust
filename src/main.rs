use std::fs;
use std::fs::*;
use std::io::prelude::*;
use std::thread::*;
use std::time::Instant;

fn main() {
    let d = Instant::now();
    let file_path = "./measurements.txt".to_string();
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    println!("Time taken for file upload: {:?}", d.elapsed());

    let available_threads = available_parallelism().unwrap();
    println!("Available threads: {:?}", available_threads);

    let file_bytes = contents.as_bytes();
    println!("File bytes: {:?}", file_bytes);

    let line_parsing_time = Instant::now();
    // for line in file_bytes.lines() {}

    println!(
        "Time taken for parsing file {:?}",
        line_parsing_time.elapsed()
    );
}
