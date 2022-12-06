use std::collections::HashSet;
use itertools::Itertools;
use crate::util;

pub fn main() {
    let input = util::parse_strings("resources/day6.txt");

    println!("Day 6, Part 1: {:?} characters in", find_packet_marker(input[0].clone()));
    println!("Day 6, Part 2: {:?} characters in", find_message_marker(input[0].clone()));
}

fn find_packet_marker(data_stream_buffer: String) -> usize {
    find_marker(data_stream_buffer, 4)
}

fn find_message_marker(data_stream_buffer: String) -> usize {
    find_marker(data_stream_buffer, 14)
}

fn find_marker(data: String, marker_length: usize) -> usize {
    for (i, window) in data
        .chars()
        .collect_vec()[..]
        .windows(marker_length)
        .enumerate() {
        let set: HashSet<&char> = HashSet::from_iter(window);
        if set.len() == marker_length {
            return i + marker_length;
        }
    }
    panic!("No marker found")
}

#[cfg(test)]
mod tests {
    use crate::day6::*;

    #[test]
    fn find_packet_marker_for_example() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string();
        assert_eq!(find_packet_marker(input), 7);
    }

    #[test]
    fn find_message_marker_for_example() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string();
        assert_eq!(find_message_marker(input), 19);
    }
}







