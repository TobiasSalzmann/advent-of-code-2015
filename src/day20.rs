use itertools::Itertools;
use crate::util;
use rayon::prelude::*;

pub fn main() {
    let input = util::parse_from_string("resources/day20.txt");

    println!("Day 20 Part 1: {:?}", find_lucky_house(input, 10, None));
    println!("Day 20 Part 2: {:?}", find_lucky_house(input, 11, Some(50)));
}


fn find_lucky_house(target_presents: usize, present_multiplier: usize, house_limit: Option<usize>) -> usize {
    for sieve_size in (0..).map(|n| 2_usize.pow(n)) {
        let mut houses = vec![present_multiplier; sieve_size];
        for elf in 2..sieve_size {
            for i in 1..house_limit.unwrap_or(sieve_size) {
                if elf * i >= sieve_size {
                    break;
                }
                houses[elf * i] += elf * present_multiplier;
            }
        }
        if let Some((idx,_item)) = houses.into_iter()
            .find_position(|ps| *ps >= target_presents) {
            return idx
        }
    }
    unreachable!()
}