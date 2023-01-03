



use itertools::{iterate, Itertools};

use crate::util;

pub fn main() {
    let input = util::parse_string("resources/day10.txt");


    println!("Day 10 Part 1: {}", repeat_look_and_say(&input, 40));
    println!("Day 10 Part 2: {}", repeat_look_and_say(&input, 50));
    // println!("Day 10 Part 2: {}", shortest_roundtrip(&input).into_option().unwrap().1);
}

fn repeat_look_and_say(string: &String, times: usize) -> usize {
    iterate(string.clone(), look_and_say)
        .take(times + 1)
        .last()
        .unwrap()
        .chars()
        .count()
}

fn look_and_say(string: &String) -> String {
    string
        .chars()
        .group_by(|x| x.clone())
        .into_iter()
        .map(|(c, seq)| format!("{}{}", seq.count(), c))
        .join("")
}