use std::cmp::Ordering;
use std::process::id;
use std::str::FromStr;
use itertools::Itertools;
use petgraph::algo::dijkstra;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::prelude::DiGraphMap;
use serde_json::{Error, Value};
use serde_json::Value::{Array, Number};
use crate::util;
use crate::util::Grid;

pub fn main() {
    let grid = util::parse_grid("resources/day12.txt", |x| x);


    println!(
        "Day 12, Part 1: {:?}", find_shortest_path_length(&grid, grid.location_of(&'S').unwrap())
    );

    println!(
        "Day 12, Part 2: {:?}", find_shortest_path_length_from_lowest(grid)
    );
}

fn find_shortest_path_length_from_lowest(grid: Grid<char>) -> i32 {
    grid.entries().iter()
        .filter(|(_, v)|*v == 'a')
        .map(|(k,_)| find_shortest_path_length(&grid, k.clone()))
        .min().unwrap()
}


fn find_shortest_path_length(grid: &Grid<char>, start: (i32, i32)) -> i32 {
    let edges: Vec<((i32, i32), (i32, i32))> = grid.edges().iter()
        .flat_map(|((vx1, c1), (vx2, c2))| {
            let mut v = vec![];
            if valid(c1, c2) {
                v.push((vx1.clone(), vx2.clone()));
            }
            if valid(c2, c1) {
                v.push((vx2.clone(), vx1.clone()));
            }
            v
        }).collect_vec();


    let g = DiGraphMap::<(i32, i32), ()>::from_edges(edges);


    let end = grid.location_of(&'E').unwrap();

    let res = dijkstra(&g, start, None, |_| 1).get(&end).unwrap_or(&100000000).clone();
    res
}

fn valid(c1: &char, c2: &char) -> bool {
    level(c1) + 1 >= level(c2)
}

fn level(c: &char) -> i32 {
    match c {
        'S' => 0,
        'E' => 25,
        _ => c.clone() as i32 - 'a' as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::day12::valid;
    use crate::day13::*;

    #[test]
    fn should_do_stuff() {
        assert!(valid(&'S',&'S'));;
    }
}







