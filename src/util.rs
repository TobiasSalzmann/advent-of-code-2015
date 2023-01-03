
use std::fmt::{Debug};
use std::fs;
use std::str::FromStr;


pub fn parse_strings(file_path: &str) -> Vec<String> {
    let contents = fs::read_to_string(file_path).expect("File does not exists");
    contents.lines()
        .map(|s| s.to_string())
        .collect()
}

pub fn parse_string(file_path: &str) -> String {
    let contents = fs::read_to_string(file_path).expect("File does not exists");
    contents.trim().to_string()
}

pub fn parse_from_strings<T: FromStr>(file_path: &str) -> Vec<T> where <T as FromStr>::Err: Debug {
    let contents = fs::read_to_string(file_path).expect("File does not exists");
    contents.lines()
        .map(|s| s.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    
    use std::io::Write;
    use crate::util::{parse_strings};
    use tempfile::NamedTempFile;

    #[test]
    fn parses_strings() {
        let mut file: NamedTempFile = NamedTempFile::new().expect("Failed to create file");
        file.write_all("This\nis a\nFile!\n".as_bytes()).expect("Failed to write to file");
        let filename = file.path().to_str().expect("Failed to get file path");

        let strings = parse_strings(filename);

        let expected_strings: Vec<String> = vec![
            "This".to_string(),
            "is a".to_string(),
            "File!".to_string(),
        ];
        assert_eq!(strings, expected_strings);
    }
}