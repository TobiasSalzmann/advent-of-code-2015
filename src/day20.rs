use std::collections::HashMap;
use std::mem;
use std::str::FromStr;
use itertools::{chain, Itertools};
use pathfinding::prelude::astar;

use rayon::prelude::*;
use crate::util;

pub fn main() {
    let input: Vec<i64> = util::parse_from_strings("resources/day20.txt");

    println!("Day 20, Part 1: {:?}", grove_coordinates(input.clone()));
    println!("Day 20, Part 2: {:?}", grove_coordinates_encrypted(input.clone(), 811589153));
}

fn mix(input: &mut Vec<(usize, i64)>) {
    let n = input.len().clone();
    for i in 0..n {
        move_value(input, i);
    }
}

fn move_value(mut vec: &mut Vec<(usize, i64)>, orig_i: usize) {
    let (i, el) = vec.iter().find_position(|el| el.0 == orig_i).unwrap();
    let (_, x) = el.clone();
    vec.remove(i);
    let new_i = (i as i64 + x).rem_euclid(vec.len() as i64) as usize;
    vec.insert(new_i, (orig_i, x));
}

fn grove_coordinates(input: Vec<i64>) -> i64 {
    let mut vec = input.into_iter().enumerate().collect_vec();
    mix(&mut vec);
    let (i, _) = vec.iter().find_position(|el| el.1 == 0).unwrap();
    let n = vec.len();
    vec[(i + 1000) % n].1 + vec[(i + 2000) % n].1 + vec[(i + 3000) % n].1
}

fn grove_coordinates_encrypted(input: Vec<i64>, key: i64) -> i64 {
    let mut vec = input.into_iter()
        .map(|x| x * key)
        .enumerate()
        .collect_vec();
    for _ in 0..10 {
        mix(&mut vec);
    }
    let (i, _) = vec.iter().find_position(|el| el.1 == 0).unwrap();
    let n = vec.len();
    vec[(i + 1000) % n].1 + vec[(i + 2000) % n].1 + vec[(i + 3000) % n].1
}

#[cfg(test)]
mod tests {
    use crate::day20::{grove_coordinates, grove_coordinates_encrypted, mix, move_value};

    #[test]
    fn should_mix_on_example() {
        let mut input: Vec<(usize, i64)> = vec![(0, 1), (1, 2), (2, -3), (3, 3), (4, -2), (5, 0), (6, 4)];
        mix(&mut input);
        assert_eq!(input, vec![(4, -2), (0, 1), (1, 2), (2, -3), (6, 4), (5, 0), (3, 3)])
    }

    #[test]
    fn should_solve_part_1_on_example() {
        let input = vec![1, 2, -3, 3, -2, 0, 4];
        assert_eq!(grove_coordinates(input), 3);
    }

    #[test]
    fn should_solve_part_2_on_example() {
        let input = vec![1, 2, -3, 3, -2, 0, 4];
        assert_eq!(grove_coordinates_encrypted(input, 811589153), 1623178306);
    }
}
