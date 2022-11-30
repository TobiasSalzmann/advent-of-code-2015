use std::fs;

pub fn parse_int_lines(file_path: &str) -> Vec<i32> {
    let contents = fs::read_to_string(file_path).expect("File does not exists");
    let lines = contents.lines();
    let numbers: Vec<i32> = lines
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    numbers
}