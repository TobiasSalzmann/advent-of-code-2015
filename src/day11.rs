




use itertools::{Itertools};
use pathfinding::num_traits::pow;

use crate::util;

pub fn main() {
    let input = util::parse_string("resources/day11.txt");


    let password = next_password(&input);
    println!("Day 11 Part 1: {}", password);
    println!("Day 11 Part 2: {}", next_password(&password));
}

fn next_password(start: &String) -> String {
    let n = start.len();
    for pw_number in from_password(start).. {
        let pw = to_password(&(pw_number % pow(26, n)), n);
        if is_valid(&pw) {
            return pw;
        }
    }
    panic!()
}

fn is_valid(pw: &String) -> bool {
    has_straight(pw) && !has_i_o_l(pw) && has_two_repeats(pw)
}

fn has_straight(pw: &String) -> bool {
    pw.bytes()
        .tuple_windows::<(u8, u8, u8)>()
        .any(|(a, b, c)| b - a == 1 && c - b == 1)
}

fn has_i_o_l(pw: &String) -> bool {
    pw.chars().any(|c| "iol".contains(c))
}

fn has_two_repeats(pw: &String) -> bool {
    let double_indices = pw.chars()
        .tuple_windows::<(char, char)>()
        .enumerate()
        .filter(|(_, (a, b))| a == b)
        .map(|(i, _)| i)
        .collect_vec();
    let has_two_repeats = match double_indices[..] {
        [] | [_] => false,
        [a, b] => a.abs_diff(b) > 1,
        _ => true
    };
    has_two_repeats
}

fn to_password(x: &u128, letters: usize) -> String {
    let mut result = vec![];
    let mut x = x.clone();

    for _ in 0..letters {
        result.push(char::from_u32('a' as u32 + ((x as u32) % 26)).unwrap());
        x = x / 26;
    }
    result.into_iter().rev().collect()
}

fn from_password(password: &String) -> u128 {
    password.chars()
        .rev()
        .enumerate()
        .map(|(i, c)| (c as u128 - 'a' as u128) * pow(26, i))
        .sum()
}