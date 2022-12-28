use std::str::FromStr;
use itertools::Itertools;
use crate::util;

pub fn main() {
    let input = util::parse_strings("resources/day3.txt").first().unwrap().clone();

    println!("Day 2, Part 1: {:?}", number_of_houses_visited(&input));
    println!("Day 2, Part 2: {:?}", number_of_houses_visited_with_robo_santa(&input));
}

fn number_of_houses_visited(directions: &String) -> usize {
    let houses = get_houses(directions);
    houses.iter().unique().count()
}

fn get_houses(directions: &String) -> Vec<(i32, i32)> {
    let mut houses = vec![(0, 0)];
    for c in directions.chars() {
        let (x, y) = houses.last().unwrap().clone();
        houses.push(match c {
            '<' => (x - 1, y),
            '>' => (x + 1, y),
            '^' => (x, y - 1),
            'v' => (x, y + 1),
            _ => panic!()
        })
    }
    houses
}

fn number_of_houses_visited_with_robo_santa(directions: &String) -> usize {
    let (dirs, robo_dirs) = directions.chars().tuples().unzip();
    get_houses(&dirs).iter()
        .chain(get_houses(&robo_dirs).iter())
        .unique().count()
}