
use itertools::{repeat_n};
use md5::Digest;
use crate::util;

pub fn main() {
    let input = util::parse_strings("resources/day4.txt").first().unwrap().clone();

    println!("Day 4, Part 1: {}", hash(&input, 5));
    println!("Day 4, Part 2: {}", hash(&input, 6));
}

fn hash(secret_key: &String, number_of_zeroes: usize) -> usize {
    let pattern: String = repeat_n('0', number_of_zeroes).collect();
    for i in 1.. {
        let s = format!("{}{}", secret_key, i);
        let digest: Digest = md5::compute(s.as_bytes());
        let encoded_string = format!("{:x}", digest);
        if encoded_string.starts_with(&pattern) {
            return i;
        }
    }
    panic!()
}