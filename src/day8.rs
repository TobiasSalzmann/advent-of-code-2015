use std::collections::HashMap;
use std::str::FromStr;
use itertools::{Itertools};

use crate::util;

pub fn main() {
    let input = util::parse_strings("resources/day8.txt");

    println!("Day 8 Part 1: {}", count_unescapes(&input));
    println!("Day 8, Part 2: {}", count_escapes(&input));
}

fn count_unescapes(escaped_strings: &Vec<String>) -> usize {
    let unescaped_strings = escaped_strings.iter()
        .map(unescape)
        .collect_vec();
    for unescaped_string in escaped_strings.iter().zip(unescaped_strings.iter()) {
        println!("[{}] [{}]", unescaped_string.0, unescaped_string.1)
    }
    let escaped_len: usize = escaped_strings.iter().map(|s| s.chars().count()).sum();
    let unescaped_len: usize = unescaped_strings.iter().map(|s| s.chars().count()).sum();
    escaped_len - unescaped_len
}

fn unescape(string: &String) -> String {
    let mut it = string
        .strip_prefix("\"").unwrap()
        .strip_suffix("\"").unwrap()
        .chars();
    let mut acc: Vec<char> = vec![];
    while let Some(c) = it.next() {
        match c {
            '\\' => match it.next() {
                Some('x') => {
                    let x1 = it.next().unwrap().to_digit(16).unwrap();
                    let x2 = it.next().unwrap().to_digit(16).unwrap();
                    acc.push(char::from_u32(16 * x1 + x2).unwrap())
                }
                Some(c) => acc.push(c),
                None => panic!(),
            }
            c => acc.push(c)
        }
    }
    acc.iter().collect()
}

fn count_escapes(strings: &Vec<String>) -> usize {
     strings.iter()
        .map(|s| s.chars().filter(|c| *c == '\\' || *c == '\"').count() + 2)
        .sum()
}