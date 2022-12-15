use std::collections::HashMap;

use std::str::FromStr;
use itertools::Itertools;

use crate::util;
use crate::util::{bounds, Bounds};

pub fn main() {
    let input: Vec<RockFormation> = util::parse_from_strings("resources/day14.txt");


    println!(
        "Day 14, Part 1: {:?}", part(input.clone(), false, false)
    );

    println!(
        "Day 14, Part 2: {:?}", part(input.clone(), true, false)
    );
}

fn part(input: Vec<RockFormation>, floor: bool, print: bool) -> usize {
    let mut map = make_map(input);
    fill(&mut map, floor);
    if print {
        print_map(&map);
    }


    let result = map.values().filter(|c| c == &&'o').count();
    result
}

fn fill(map: &mut HashMap<(i32, i32), char>, floor: bool) {
    let abyss = bounds(map).max_y + 2;
    let mut sand_vec = vec![(500, 0)];
    while extend(&mut sand_vec, map, abyss, floor) {
        let drop_point = match sand_vec.pop() {
            Some(p) => p,
            None => return
        };
        map.insert(drop_point, 'o');
    }
}

fn extend(sand_vec: &mut Vec<(i32, i32)>, map: &HashMap<(i32, i32), char>, abyss: i32, floor: bool) -> bool {
    while let Some(Some(p)) = &sand_vec.last().map(|r| next_empty(r, map, abyss, floor)) {
        sand_vec.push(p.clone());
        if p.1 > abyss {
            return false;
        }
    }
    true
}

fn next_empty(p: &(i32, i32), map: &HashMap<(i32, i32), char>, abyss: i32, floor: bool) -> Option<(i32, i32)> {
    let (x, y) = p.clone();
    [(x, y+1), (x-1, y+1), (x+1, y+1)]
        .into_iter()
        .filter(|p| !map.contains_key(p) && (!floor || p.1 < abyss))
        .next()
}

fn make_map(formations: Vec<RockFormation>) -> HashMap<(i32, i32), char> {
    let mut map = HashMap::new();
    for formation in formations {
        for rock_location in formation.rock_locations() {
            map.insert(rock_location, '#');
        }
    }
    map
}

fn print_map(map: &HashMap<(i32, i32), char>) {
    let Bounds { min_x, max_x, min_y, max_y } = bounds(map);
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            print!("{}", map.get(&(x, y)).map(|v| v.to_string()).unwrap_or(".".to_string()));
        }
        println!();
    }
}

#[derive(Debug, Clone)]
struct RockFormation {
    inner: Vec<(i32, i32)>,
}

impl RockFormation {
    fn rock_locations(self) -> Vec<(i32, i32)> {
        let windows = self.inner.into_iter()
            .tuple_windows::<((i32, i32), (i32, i32))>();
        windows.flat_map(|(a, b)| { connect(a, b) })
            .collect_vec()
    }
}

fn connect(a: (i32, i32), b: (i32, i32)) -> Vec<(i32, i32)> {
    let mut vec = vec![];
    let dx = (b.0 - a.0).signum();
    let dy = (b.1 - a.1).signum();

    for i in 0.. {
        let v = (a.0 + i * dx, a.1 + i * dy);
        vec.push(v);

        if v == b {
            break;
        }
    }
    vec
}

impl FromStr for RockFormation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner = s.split(" -> ").map(|raw_point| {
            let (raw_x, raw_y) = raw_point.split_once(',').unwrap();
            (raw_x.parse().unwrap(), raw_y.parse().unwrap())
        }).collect_vec();
        Ok(RockFormation { inner })
    }
}

#[cfg(test)]
mod tests {
    

    #[test]
    fn should_do_stuff() {}
}







