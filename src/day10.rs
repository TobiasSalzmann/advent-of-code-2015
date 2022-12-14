use std::str::FromStr;
use itertools::{fold, Itertools};
use crate::day10::Command::{AddX, NoOp};
use crate::util;

pub fn main() {
    let input: Vec<Command> = util::parse_from_strings("resources/day10.txt");

    println!(
        "Day 11, Part 1: {:?}", part1(input.clone())
    );

    println!(
        "Day 11, Part 2:\n{}", part2(input)
    );
}

fn part1(commands: Vec<Command>) -> i32 {
    let v = compute_signal(commands);
    [20, 60, 100, 140, 180, 220].iter()
        .map(|i| i * v[i.clone() as usize - 1])
        .sum()
}

fn compute_signal(commands: Vec<Command>) -> Vec<i32> {
    let mut v = vec![1];
    for command in commands {
        let last = v.last().unwrap().clone();
        v.push(last.clone());
        match command {
            NoOp => {}
            AddX(inc) => v.push(last + inc)
        }
    }
    v
}

fn part2(commands: Vec<Command>) -> String {
    let v = compute_signal(commands);

    v.iter()
        .take(240)
        .enumerate()
        .map(|(i, n)| {
            let j = (i as i32) % 40;
            if j >= n-1 && j <= n+1 {'#'} else {'.'}
        })
        .chunks(40).into_iter()
        .map(|cs| cs.into_iter().join(""))
        .join("\n")
}

#[derive(Clone)]
enum Command {
    NoOp,
    AddX(i32)
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s.split(" ").collect_vec()[..] {
            ["noop"] => Ok(NoOp),
            ["addx", num] => Ok(AddX(num.parse().unwrap())),
            v => panic!("unexpected value {:?}",v)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day13::*;

    #[test]
    fn should_do_stuff() {}
}







