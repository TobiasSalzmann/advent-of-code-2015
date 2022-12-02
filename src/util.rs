use std::fs;

// pub fn parse_int_lines(file_path: &str) -> Vec<i32> {
//     let contents = fs::read_to_string(file_path).expect("File does not exists");
//     let lines = contents.lines();
//     let numbers: Vec<i32> = lines
//         .filter_map(|s| s.parse::<i32>().ok())
//         .collect();
//     numbers
// }

pub fn parse_int_lists(file_path: &str) -> Vec<Vec<i32>> {
    let contents = fs::read_to_string(file_path).expect("File does not exists");
    let lines: Vec<&str> = contents.lines()
        .collect();
    lines.split(|s| s.is_empty())
        .map(|ss| ss.iter().filter_map(|s| s.parse::<i32>().ok()).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;
    use crate::util::{parse_int_lists};
    use tempfile::NamedTempFile;

    #[test]
    fn parses_list() {

        let mut file: NamedTempFile = NamedTempFile::new().expect("Failed to create file");
        file.write_all("43\n\n2\n20\n22\n\n1\n1\n1\n1\n1\n".as_bytes()).expect("Faile to write to file");
        let filename = file.path().to_str().expect("Failed to get file path");

        let lists = parse_int_lists(filename);

        let expected_lists: Vec<Vec<i32>> = vec![
            vec![43],
            vec![2, 20, 22],
            vec![1, 1, 1, 1, 1],
        ];

        assert_eq!(lists, expected_lists);
    }
}