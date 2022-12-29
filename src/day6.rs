use std::str::FromStr;
use itertools::{Itertools};


use crate::day6::Action::{Toggle, TurnOff, TurnOn};
use crate::util;

pub fn main() {
    let input = util::parse_from_strings("resources/day6.txt");

    println!("Day 6, Part 1: {}", count_lit(&input));
    println!("Day 6, Part 2: {}", count_total_brightness(&input));
}

fn count_lit(instructions: &Vec<Instruction>) -> usize {
    let mut arr = [[false; 1000]; 1000];
    for instruction in instructions {
        for x in instruction.start.0..=instruction.end.0 {
            for y in instruction.start.1..=instruction.end.1 {
                match instruction.action {
                    TurnOn => { arr[x][y] = true }
                    TurnOff => { arr[x][y] = false }
                    Toggle => { arr[x][y] = !arr[x][y] }
                }
            }
        }
    }
    let mut count = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            if arr[x][y] { count += 1 }
        }
    }
    count
}

fn count_total_brightness(instructions: &Vec<Instruction>) -> usize {
    let mut arr = [[0; 1000]; 1000];
    for instruction in instructions {
        for x in instruction.start.0..=instruction.end.0 {
            for y in instruction.start.1..=instruction.end.1 {
                match instruction.action {
                    TurnOn => { arr[x][y] += 1 }
                    TurnOff => { if arr[x][y] > 0 { arr[x][y] -= 1 } }
                    Toggle => { arr[x][y] += 2 }
                }
            }
        }
    }
    let mut count = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            count += arr[x][y]
        }
    }
    count
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
struct Instruction {
    start: (usize, usize),
    end: (usize, usize),
    action: Action,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match &s.replace(',', " ").split_whitespace().collect_vec()[..] {
            ["turn", "on", x1, y1, "through", x2, y2] => Instruction {
                start: (x1.parse().unwrap(), y1.parse().unwrap()),
                end: (x2.parse().unwrap(), y2.parse().unwrap()),
                action: TurnOn,
            },
            ["turn", "off", x1, y1, "through", x2, y2] => Instruction {
                start: (x1.parse().unwrap(), y1.parse().unwrap()),
                end: (x2.parse().unwrap(), y2.parse().unwrap()),
                action: TurnOff,
            },
            ["toggle", x1, y1, "through", x2, y2] => Instruction {
                start: (x1.parse().unwrap(), y1.parse().unwrap()),
                end: (x2.parse().unwrap(), y2.parse().unwrap()),
                action: Toggle,
            },
            _ => panic!()
        })
    }
}