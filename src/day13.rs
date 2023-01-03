use std::collections::{HashMap};
use std::str::FromStr;
use itertools::{Itertools};

use crate::util;

pub fn main() {
    let input = util::parse_from_strings("resources/day13.txt");


    println!("Day 13 Part 1: {}", maximize_total_happiness(&input, false));
    println!("Day 13 Part 2: {}", maximize_total_happiness(&input, true));
}

fn maximize_total_happiness(input: &Vec<Happiness>, include_self: bool) -> i32 {
    let lookup: HashMap<(String, String), i32> = input
        .iter()
        .map(|Happiness { name, other, gain }| ((name.clone(), other.clone()), gain.clone()))
        .collect();
    let names = lookup
        .keys()
        .map(|(a, _)| a)
        .unique()
        .cloned()
        .collect_vec();
    if include_self {
        let mut including_self = input.clone();
        for name in names {
            including_self.push(Happiness{
                name: name.clone(),
                other: "self".to_string(),
                gain: 0,
            });
            including_self.push(Happiness{
                name: "self".to_string(),
                other: name.clone(),
                gain: 0,
            });
        }
        return maximize_total_happiness(&including_self, false)
    }

    names
        .iter()
        .permutations(names.len())
        .map(|v| {
            let mut sum = 0;
            for i in 0..v.len() {
                let a = v[i];
                let b = v[(i + 1) % v.len()];
                sum += lookup.get(&(a.clone(), b.clone())).unwrap() + lookup.get(&(b.clone(), a.clone())).unwrap()
            }
            sum
        })
        .max().unwrap()
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Debug)]
struct Happiness {
    name: String,
    other: String,
    gain: i32,
}

impl FromStr for Happiness {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let [name, "would", direction, magnitude, "happiness", "units", "by", "sitting", "next", "to", other] = s.split_whitespace().collect_vec()[..] {
            return Ok(Happiness {
                name: name.to_string(),
                other: other.strip_suffix(".").unwrap().to_string(),
                gain: magnitude.parse::<i32>().unwrap() * if direction == "gain" { 1 } else { -1 },
            });
        }
        panic!()
    }
}