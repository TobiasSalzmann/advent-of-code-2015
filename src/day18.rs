use std::collections::HashSet;

use crate::util;

type Light = (i32, i32);

pub fn main() {
    let input = util::parse_strings("resources/day18.txt");
    let lights: HashSet<Light> = parse(&input);

    println!("Day 18 Part 1: {}", simulate_steps(&lights, 100, 100, 100));
    // println!("Day 17 Part 2: {}", find_minimal_number_of_combinations(&input, 150));
}

fn simulate_steps(lights: &HashSet<Light>, steps: usize, max_x: i32, max_y: i32) -> usize {
    let corners: HashSet<(i32, i32)> = HashSet::from([(0, 0), (0, max_y - 1), (max_x - 1, 0), (max_x - 1, max_y - 1)]);
    let mut lights = lights
        .union(&corners)
        .cloned()
        .collect();
    for _ in 0..steps {
        let set = step(lights, max_x, max_y);
        lights = set.union(&corners)
            .cloned()
            .collect();
    }
    lights.len()
}

fn step(lights: HashSet<Light>, max_x: i32, max_y: i32) -> HashSet<Light> {
    let potential_lights: HashSet<Light> = lights.iter()
        .flat_map(neighbours)
        .filter(|(x,y)| *x >= 0 && *x < max_x && *y >= 0 && *y < max_y)
        .collect();
    potential_lights.into_iter()
        .filter(|light| on_next(light, lights.contains(light), &lights))
        .collect()

}

fn on_next(light: &Light, on: bool, potential_lights: &HashSet<Light>) -> bool {
    let actual_neighbours = neighbours(light)
        .intersection(potential_lights)
        .count();
    match actual_neighbours {
        3 => true,
        2 if on => true,
        _ => false,
    }
}

fn neighbours(light: &Light) -> HashSet<Light> {
    let mut lights = HashSet::new();
    for x in [-1, 0, 1] {
        for y in [-1, 0, 1] {
            if x != 0 || y != 0 {
                lights.insert((light.0 + x, light.1 + y));
            }
        }
    }
    lights
}

fn parse(input: &Vec<String>) -> HashSet<Light> {
    let mut lights = HashSet::new();
    for (y, row) in input.iter().enumerate() {
        for (x, c) in row.chars().enumerate() {
            if c == '#' {
                lights.insert((x as i32, y as i32));
            }
        }
    }
    lights
}

