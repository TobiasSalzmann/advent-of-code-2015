use std::collections::{HashMap, HashSet};
use itertools::{Itertools};
use pathfinding::prelude::dijkstra;
use crate::util;

pub fn main() {
    let input: Vec<String> = util::parse_strings("resources/day24.txt");
    let basin = parse(input);

    println!("Day 24, Part 1: {:?}", minimum_rounds(&basin));
    println!("Day 24, Part 2: {:?}", minimum_rounds_part_2(&basin));
}

fn minimum_rounds_part_2(basin: &Basin) -> usize {
    let initial = State { x: 0, y: -1, blizzard_cycle: 0 };
    let empty_locations_by_cycle = generate_empty_locations_by_cycle(basin);

    let (path, forward_cost) = dijkstra(
        &initial,
        |state| successors(state, &empty_locations_by_cycle, basin),
        |s| is_exit_state(s, basin),
    ).unwrap();

    let (path, return_cost) = dijkstra(
        path.last().unwrap(),
        |state| successors(state, &empty_locations_by_cycle, basin),
        |s| is_entrance_state(s),
    ).unwrap();

    let (_path, forward_again_cost) = dijkstra(
        path.last().unwrap(),
        |state| successors(state, &empty_locations_by_cycle, basin),
        |s| is_exit_state(s, basin),
    ).unwrap();

    forward_cost + return_cost + forward_again_cost
}

fn minimum_rounds(basin: &Basin) -> usize {
    let initial = State {
        x: 0,
        y: -1,
        blizzard_cycle: 0,
    };

    let empty_locations_by_cycle = generate_empty_locations_by_cycle(basin);

    let (_path, cost) = dijkstra(
        &initial,
        |n| successors(n, &empty_locations_by_cycle, basin),
        |s| is_exit_state(s, basin),
    ).unwrap();

    cost
}

fn generate_empty_locations_by_cycle(basin: &Basin) -> HashMap<i32, HashSet<(i32, i32)>> {
    let mut all_locations = HashSet::new();
    for x in 0..basin.width {
        for y in 0..basin.height {
            all_locations.insert((x, y));
        }
    }

    let number_of_cycles = (1..)
        .find(|n| n % basin.height == 0 && n % basin.width == 0)
        .unwrap();

    (0..number_of_cycles)
        .map(|n| {
            let mut empty_locations = all_locations.clone();
            for Blizzard { x, y, dx, dy } in &basin.blizzards {
                let new_x = (x + (n * dx)).rem_euclid(basin.width);
                let new_y = (y + (n * dy)).rem_euclid(basin.height);
                empty_locations.remove(&(new_x, new_y));
            }
            (n, empty_locations)
        }).collect()
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
        .map(|(x, y)| (State { x, y, blizzard_cycle }, 1))
        .filter(|(neighbour, _)| {
            let is_empty = empty_locations_by_cycle
                .get(&blizzard_cycle).unwrap()
                .contains(&(neighbour.x, neighbour.y));
            is_entrance_state(neighbour)
                || is_exit_state(neighbour, basin)
                || (is_in_bounds(neighbour, basin) && is_empty)
        })
        .collect_vec()
}

fn is_exit_state(state: &State, basin: &Basin) -> bool {
    state.x == basin.width - 1 && state.y == basin.height
}

fn is_entrance_state(state: &State) -> bool {
    state.x == 0 && state.y == -1
}

fn is_in_bounds(state: &State, basin: &Basin) -> bool {
    state.x >= 0
        && state.x < basin.width
        && state.y >= 0
        && state.y < basin.height
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
    use crate::day24::{minimum_rounds, minimum_rounds_part_2, parse};

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
        assert_eq!(minimum_rounds(&parse(input)), 18)
    }

    #[test]
    fn should_solve_part_2_on_example() {
        let input = vec![
            "#.######".to_string(),
            "#>>.<^<#".to_string(),
            "#.<..<<#".to_string(),
            "#>v.><>#".to_string(),
            "#<^v^^>#".to_string(),
            "######.#".to_string(),
        ];
        assert_eq!(minimum_rounds_part_2(&parse(input)), 54)
    }
}





