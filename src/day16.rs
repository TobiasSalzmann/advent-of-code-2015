use std::collections::HashMap;

use std::str::FromStr;
use itertools::{Itertools};

use crate::util;

pub fn main() {
    let input = util::parse_from_strings("resources/day16.txt");
    let compounds: Vec<(String, u32)> = vec![
        ("children".to_string(), 3),
        ("cats".to_string(), 7),
        ("samoyeds".to_string(), 2),
        ("pomeranians".to_string(), 3),
        ("akitas".to_string(), 0),
        ("vizslas".to_string(), 0),
        ("goldfish".to_string(), 5),
        ("trees".to_string(), 3),
        ("cars".to_string(), 2),
        ("perfumes".to_string(), 1),
    ];
    let sue = Sue {
        id: 0,
        compounds: compounds.into_iter().collect(),
    };

    println!("Day 15 Part 1: {}", find_id(&input, &sue));
    println!("Day 15 Part 2: {}", find_id_2(&input, &sue));
}

fn find_id(sues: &Vec<Sue>, sue: &Sue) -> u32 {
    sues.iter().find(|s| {
        for (k, v) in s.compounds.iter() {
            if sue.compounds.get(k).unwrap() != v {
                return false;
            }
        }
        true
    }).unwrap().id
}

fn find_id_2(sues: &Vec<Sue>, sue: &Sue) -> u32 {
    sues.iter().find(|s| {
        for (k, v) in s.compounds.iter() {
            let computed_v = sue.compounds.get(k).unwrap();
            let matches = match k.as_str() {
                "cats" | "trees" => v > computed_v,
                "pomeranians" | "goldfish" => v < computed_v,
                _ => v == computed_v
            };
            if !matches {
                return false;
            }
        }
        true
    }).unwrap().id
}

#[derive(Clone, Debug)]
struct Sue {
    id: u32,
    compounds: HashMap<String, u32>,
}

impl FromStr for Sue {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pairs = s.split_whitespace()
            .map(|s| s.trim_end_matches([',', ':']))
            .tuples::<(&str, &str)>().collect_vec();
        let mut compounds = HashMap::new();
        let mut id = 0;
        for (a, b) in pairs {
            let n = b.parse::<u32>().unwrap();
            match a {
                "Sue" => id = n,
                _ => { compounds.insert(a.to_string(), n); }
            };
        }

        Ok(Sue { id, compounds })
    }
}