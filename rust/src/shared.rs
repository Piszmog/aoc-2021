use std::env;
use std::error::Error;
use std::time::Instant;

pub fn get_args() -> Vec<String> {
    env::args().collect()
}

pub fn get_csv_reader(file_path: &str, has_headers: bool, delimiter: u8) -> Result<csv::Reader<std::fs::File>, Box<dyn Error>> {
    let file = std::fs::File::open(file_path)?;
    let reader = csv::ReaderBuilder::new()
        .has_headers(has_headers)
        .delimiter(delimiter)
        .from_reader(file);
    Ok(reader)
}

pub fn print_duration(start: Instant) {
    let duration = Instant::now().duration_since(start);
    println!("Duration: {:?}", duration);
}