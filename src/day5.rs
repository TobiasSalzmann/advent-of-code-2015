
use itertools::{Itertools};

use crate::util;

pub fn main() {
    let input = util::parse_strings("resources/day5.txt");

    println!("Day 5, Part 1: {}", count_nice_strings(input.clone()));
    println!("Day 5, Part 2: {}", count_nice_strings_2(input.clone()));
}

fn count_nice_strings(strings: Vec<String>) -> usize {
    strings.into_iter()
        .filter(is_nice)
        .count()
}

fn count_nice_strings_2(strings: Vec<String>) -> usize {
    strings.into_iter()
        .filter(is_nice_2)
        .count()
}

fn is_nice(string: &String) -> bool {
    let has_three_vowels = string.chars()
        .filter(|s| "aeiou".contains(*s))
        .count() >= 3;
    let contains_repeated_letter = string.chars()
        .tuple_windows::<(char, char)>()
        .find(|(a, b)| a == b)
        .is_some();
    let contains_banned_string = ["ab", "cd", "pq", "xy"].iter()
        .any(|p| string.contains(p));
    has_three_vowels && contains_repeated_letter && !contains_banned_string
}

fn is_nice_2(string: &String) -> bool {
    let contains_sandwich = string.chars()
        .tuple_windows::<(char, char, char)>()
        .find(|(a, _, b)| a == b)
        .is_some();

    let pairs_indexed = string.chars()
        .tuple_windows::<(char, char)>()
        .enumerate()
        .map(|(a, b)| (b, a))
        .into_group_map();

    let has_separate_pairs = pairs_indexed
        .values()
        .find(|indices| {
            indices.len() > 2 || indices.len() == 2 && indices[0].abs_diff(indices[1]) > 1
        }).is_some();
    contains_sandwich && has_separate_pairs
}

#[cfg(test)]
mod test {
    use crate::day5::{is_nice, is_nice_2};

    #[test]
    fn nice_and_naughty() {
        assert!(is_nice(&"ugknbfddgicrmopn".to_string()));
        assert!(is_nice(&"aaa".to_string()));
        assert!(!is_nice(&"jchzalrnumimnmhp".to_string()));
        assert!(!is_nice(&"haegwjzuvuyypxyu".to_string()));
        assert!(!is_nice(&"dvszwmarrgswjxmb".to_string()));
    }

    #[test]
    fn nice_and_naughty_2() {
        assert!(is_nice_2(&"qjhvhtzxzqqjkmpb".to_string()));
    }
}