use itertools::{Itertools};

use crate::util;

pub fn main() {
    let input = util::parse_from_strings("resources/day17.txt");

    println!("Day 17 Part 1: {}", count_combinations(&input, 150));
    println!("Day 17 Part 2: {}", find_minimal_number_of_combinations(&input, 150));
}

fn count_combinations(containers: &Vec<u32>, target: u32) -> usize {
    containers.into_iter()
        .powerset()
        .filter(|cs| cs.iter().cloned().cloned().sum::<u32>() == target)
        .count()
}

fn find_minimal_number_of_combinations(containers: &Vec<u32>, target: u32) -> usize {
    let min_size = containers.iter()
        .cloned()
        .powerset()
        .filter(|cs| cs.iter().sum::<u32>() == target)
        .map(|cs| cs.len())
        .min()
        .unwrap();
    containers.iter()
        .cloned()
        .powerset()
        .filter(|cs| cs.iter().sum::<u32>() == target && cs.len() == min_size)
        .count()
}