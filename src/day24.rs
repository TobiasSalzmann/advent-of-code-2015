use std::collections::{HashMap, HashSet};
use itertools::{Itertools};
use pathfinding::prelude::dijkstra;
use crate::util;

pub fn main() {
    let input: Vec<String> = util::parse_strings("resources/day24.txt");
    let basin = parse(input);

    println!("Day 24, Part 1: {:?}", minimum_rounds(basin));
    // println!("Day 23, Part 2: {:?}", first_stable_round(initial_elves));
}

fn minimum_rounds(basin: Basin) -> usize {
    let initial = State {
        x: 0,
        y: -1,
        blizzard_cycle: 0,
    };

    let mut all_locations = HashSet::new();
    for x in 0..basin.width {
        for y in 0..basin.height {
            all_locations.insert((x, y));
        }
    }

    let empty_locations_by_cycle: HashMap<i32, HashSet<(i32, i32)>> = (0..(basin.width * basin.height))
        .map(|n| {
            let mut empty_locations = all_locations.clone();
            for Blizzard{x, y, dx, dy} in &basin.blizzards {
                let new_x = (x + (n * dx)).rem_euclid(basin.width);
                let new_y = (y + (n * dy)).rem_euclid(basin.height);
                empty_locations.remove(&(new_x, new_y));
            }
            (n, empty_locations)
        }).collect();

    // for entry in empty_locations_by_cycle.iter()  {
    //     println!("{:?}, {:?}", entry.0, entry.1)
    // }

    let result: (Vec<State>, usize) = dijkstra(
        &initial,
        |n| successors(n, &empty_locations_by_cycle, &basin),
        |s| s.y == basin.height - 1 && s.x == basin.width - 1,
    ).unwrap();

    result.1 + 1
}

fn successors(state: &State, empty_locations_by_cycle: &HashMap<i32, HashSet<(i32, i32)>>, basin: &Basin) -> Vec<(State, usize)> {
    let num_cycles = empty_locations_by_cycle.len();
    let blizzard_cycle = (state.blizzard_cycle + 1) % num_cycles as i32;
    let neighbours = [
        (state.x + 1, state.y),
        (state.x - 1, state.y),
        (state.x, state.y - 1),
        (state.x, state.y + 1),
        (state.x, state.y)
    ];
    neighbours.into_iter()
        .filter(|p @ (x, y)| {
            let is_origin = *x == 0 && *y == -1;
            let in_bounds = *x >= 0 && *x < basin.width && *y >= 0 && *y < basin.height;
            let is_empty = empty_locations_by_cycle
                .get(&blizzard_cycle).unwrap().contains(p);
            is_origin || (in_bounds && is_empty)
        })
        .map(|(x, y)| (State { x, y, blizzard_cycle }, 1))
        .collect_vec()
}

fn parse(raw: Vec<String>) -> Basin {
    let mut blizzards = vec![];
    for (y, line) in raw.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '^' {
                blizzards.push(Blizzard { x: (x - 1) as i32, y: (y - 1) as i32, dx: 0, dy: -1 });
            }
            if c == 'v' {
                blizzards.push(Blizzard { x: (x - 1) as i32, y: (y - 1) as i32, dx: 0, dy: 1 });
            }
            if c == '<' {
                blizzards.push(Blizzard { x: (x - 1) as i32, y: (y - 1) as i32, dx: -1, dy: 0 });
            }
            if c == '>' {
                blizzards.push(Blizzard { x: (x - 1) as i32, y: (y - 1) as i32, dx: 1, dy: 0 });
            }
        }
    }
    let width = (raw[0].len() - 2) as i32;
    let height = (raw.len() - 2) as i32;
    Basin {
        width,
        height,
        blizzards,
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Clone)]
struct State {
    x: i32,
    y: i32,
    blizzard_cycle: i32,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Clone)]
struct Basin {
    width: i32,
    height: i32,
    blizzards: Vec<Blizzard>,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Clone)]
struct Blizzard {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

#[cfg(test)]
mod tests {
    use crate::day24::{minimum_rounds, parse};

    #[test]
    fn should_solve_example() {
        let input = vec![
            "#.######".to_string(),
            "#>>.<^<#".to_string(),
            "#.<..<<#".to_string(),
            "#>v.><>#".to_string(),
            "#<^v^^>#".to_string(),
            "######.#".to_string(),
        ];
        assert_eq!(minimum_rounds(parse(input)), 18)
    }
}





