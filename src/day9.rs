use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use itertools::{Itertools, MinMaxResult};

use crate::util;

pub fn main() {
    let input = util::parse_from_strings("resources/day9.txt");


    println!("Day 9 Part 1: {}", shortest_roundtrip(&input).into_option().unwrap().0);
    println!("Day 9 Part 2: {}", shortest_roundtrip(&input).into_option().unwrap().1);
}

fn shortest_roundtrip(connections: &Vec<Connection>) -> MinMaxResult<u32> {
    let lookup: HashMap<(String, String), u32> = connections
        .iter()
        .flat_map(|Connection { a, b, cost }| [
            ((a.clone(), b.clone()), cost.clone()),
            ((b.clone(), a.clone()), cost.clone())
        ])
        .collect();
    let locations = connections
        .iter()
        .flat_map(|Connection { a, b, .. }| [a, b])
        .unique()
        .cloned()
        .collect_vec();
    locations
        .iter()
        .permutations(locations.len())
        .map(|perm| perm.
            into_iter()
            .cloned()
            .tuple_windows::<(String, String)>()
            .filter_map(|pair| lookup.get(&pair))
            .sum()
        ).minmax()
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Debug)]
struct Connection {
    a: String,
    b: String,
    cost: u32,
}

impl FromStr for Connection {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let [a, "to", b, "=", cost] = s.split_whitespace().collect_vec()[..] {
            return Ok(Connection {
                a: a.to_string(),
                b: b.to_string(),
                cost: cost.parse().unwrap(),
            })
        }
        panic!()
    }
}