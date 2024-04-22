use std::fs::*;
use std::io::prelude::*;
use std::thread::*;
use std::time::Instant;

fn process_data(data: &[u8]) {
    // println!("data = {:?}", std::str::from_utf8(&data[]));
    println!("process data = {:?}", data.len());
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

    let d1 = Instant::now();

    let file_bytes = contents.as_bytes();

    println!(
        "Time taken for file changing from string to byte : {:?}",
        d1.elapsed()
    );

    let file_bytes_size = file_bytes.len();
    println!("file bytes size: {:?}", file_bytes_size);

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

        // process_data(&file_bytes[start_index..end_index]);
        data_vec.push(&file_bytes[start_index..end_index]);

        start_index = end_index + 1;
        end_index = start_index + min_slice_size;
    }

    println!("data_vec: {}", data_vec[0].len());

    scope(|s| {
        for data in data_vec {
            // s.spawn(move || {})
            // println!("{}", std::str::from_utf8(&data[0..10]).unwrap());
            s.spawn(move || process_data(data));
        }
    })
}
