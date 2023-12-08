use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};

pub fn load_input_file(file_name: &str) -> impl Iterator<Item = String> {
    let input_file = OpenOptions::new().read(true).open(file_name).unwrap();
    let input_content = BufReader::new(input_file);
    input_content.lines().flatten()
}
