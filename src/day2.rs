use std::str::FromStr;
use itertools::Itertools;
use crate::util;

pub fn main() {
    let input = util::parse_from_strings("resources/day2.txt");

    println!("Day 2, Part 1: {:?}", amount_of_paper(&input));
    println!("Day 2, Part 2: {:?}", amount_of_ribbon(&input));
}

fn amount_of_paper(packages: &Vec<Package>) -> u32 {
    packages.iter().map(|Package { x, y, z }| {
        let mut sides = [x * y, x * z, y * z];
        sides.sort();
        sides[0] * 3 + sides[1] * 2 + sides[2] * 2
    }).sum()
}

fn amount_of_ribbon(packages: &Vec<Package>) -> u32 {
    packages.iter().map(|Package { x, y, z }| {
        let (a, b, c) = [*x, *y, *z].into_iter()
            .sorted()
            .collect_tuple()
            .unwrap();
        (a + a + b + b + a * b * c)
    }).sum()
}

struct Package {
    x: u32,
    y: u32,
    z: u32,
}

impl FromStr for Package {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y, z) = s.split('x')
            .map(|x| x.parse().unwrap())
            .collect_tuple()
            .unwrap();

        Ok(Package { x, y, z })
    }
}