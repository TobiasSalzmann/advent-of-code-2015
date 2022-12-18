use std::collections::HashSet;
use itertools::Itertools;
use petgraph::algo::kosaraju_scc;
use petgraph::graphmap::UnGraphMap;

use crate::util;

pub fn main() {
    let input: Vec<String> = util::parse_strings("resources/day18.txt");

    println!("Day 18, Part 1: {:?}", calculate_surface_area(input.clone()));
    println!("Day 18, Part 2: {:?}", calculate_exterior_surface_area(input.clone()));
}

type Droplet = (i32, i32, i32);

fn calculate_surface_area(raw_droplets: Vec<String>) -> usize {
    let droplets = droplets_hashset(raw_droplets);
    droplets.iter()
        .flat_map(neighbours)
        .filter(|d| !droplets.contains(d))
        .count()
}

fn droplets_hashset(raw_droplets: Vec<String>) -> HashSet<Droplet> {
    let droplets: HashSet<Droplet> = raw_droplets.iter()
        .map(parse_droplet)
        .collect();
    droplets
}

fn calculate_exterior_surface_area(raw_droplets: Vec<String>) -> usize {
    let droplets = droplets_hashset(raw_droplets);
    let surrounds = surrounds(&droplets);
    let exterior_point = (surrounds.min_x, surrounds.min_y, surrounds.min_z);
    let negative_graph = create_negative_graph(droplets.clone(), surrounds);
    let connected_components = kosaraju_scc(&negative_graph);
    let outside: HashSet<Droplet> = connected_components.into_iter()
        .find(|cc| cc.contains(&exterior_point)).unwrap()
        .iter().cloned().collect();
    droplets.iter()
        .flat_map(neighbours)
        .filter(|d| outside.contains(d))
        .count()
}

fn create_negative_graph(droplets: HashSet<Droplet>, surrounds: Surrounds) -> UnGraphMap<Droplet, ()> {
    let mut exterior_droplets_graph: UnGraphMap<Droplet, ()> = UnGraphMap::new();
    for x in surrounds.min_x..=surrounds.max_x {
        for y in surrounds.min_y..=surrounds.max_y {
            for z in surrounds.min_z..=surrounds.max_z {
                let node = (x, y, z);
                if !droplets.contains(&node) {
                    if !droplets.contains(&(x + 1, y, z)) {
                        exterior_droplets_graph.add_edge(node, (x + 1, y, z), ());
                    }
                    if !droplets.contains(&(x, y + 1, z)) {
                        exterior_droplets_graph.add_edge(node, (x, y + 1, z), ());
                    }
                    if !droplets.contains(&(x, y, z + 1)) {
                        exterior_droplets_graph.add_edge(node, (x, y, z + 1), ());
                    }
                }
            }
        }
    };
    exterior_droplets_graph
}

fn neighbours(droplet: &Droplet) -> Vec<Droplet> {
    vec![
        (droplet.0 - 1, droplet.1, droplet.2),
        (droplet.0 + 1, droplet.1, droplet.2),
        (droplet.0, droplet.1 - 1, droplet.2),
        (droplet.0, droplet.1 + 1, droplet.2),
        (droplet.0, droplet.1, droplet.2 - 1),
        (droplet.0, droplet.1, droplet.2 + 1),
    ]
}

fn parse_droplet(raw_droplet: &String) -> Droplet {
    raw_droplet.split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect_tuple()
        .unwrap()
}

fn surrounds(inner: &HashSet<Droplet>) -> Surrounds {
    let min_x = inner.into_iter().map(|d| d.0).min().unwrap() - 1;
    let max_x = inner.into_iter().map(|d| d.0).max().unwrap() + 1;
    let min_y = inner.into_iter().map(|d| d.1).min().unwrap() - 1;
    let max_y = inner.into_iter().map(|d| d.1).max().unwrap() + 1;
    let min_z = inner.into_iter().map(|d| d.2).min().unwrap() - 1;
    let max_z = inner.into_iter().map(|d| d.2).max().unwrap() + 1;
    Surrounds {
        min_x,
        max_x,
        min_y,
        max_y,
        min_z,
        max_z,
    }
}

#[derive(Debug)]
struct Surrounds {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    min_z: i32,
    max_z: i32,
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use crate::day18::*;

    #[test]
    fn should_solve_part_1_on_example() {
        let input = vec![
            "2,2,2".to_string(),
            "1,2,2".to_string(),
            "3,2,2".to_string(),
            "2,1,2".to_string(),
            "2,3,2".to_string(),
            "2,2,1".to_string(),
            "2,2,3".to_string(),
            "2,2,4".to_string(),
            "2,2,6".to_string(),
            "1,2,5".to_string(),
            "3,2,5".to_string(),
            "2,1,5".to_string(),
            "2,3,5".to_string(),
        ];

        assert_eq!(calculate_surface_area(input), 64);
    }

    #[test]
    fn should_solve_simple_example() {
        let input = vec![
            "0,0,0".to_string(),
            "0,0,2".to_string(),
        ];

        assert_eq!(calculate_surface_area(input), 12);
    }

    #[test]
    fn should_solve_part_2_on_example() {
        let input = vec![
            "2,2,2".to_string(),
            "1,2,2".to_string(),
            "3,2,2".to_string(),
            "2,1,2".to_string(),
            "2,3,2".to_string(),
            "2,2,1".to_string(),
            "2,2,3".to_string(),
            "2,2,4".to_string(),
            "2,2,6".to_string(),
            "1,2,5".to_string(),
            "3,2,5".to_string(),
            "2,1,5".to_string(),
            "2,3,5".to_string(),
        ];

        assert_eq!(calculate_exterior_surface_area(input), 58);
    }
}
