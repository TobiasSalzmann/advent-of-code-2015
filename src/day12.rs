use std::cmp::Ordering;
use std::collections::HashMap;
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
        "Day 12, Part 1: {:?}", find_shortest_path_length_from_any('S', &grid)
    );

    println!(
        "Day 12, Part 2: {:?}", find_shortest_path_length_from_any('a', &grid)
    );
}

fn find_shortest_path_length_from_any(start: char, grid: &Grid<char>) -> i32 {
    find_shortest_path_lengths(grid).iter()
        .filter(|((x, y), _)| grid.value_at(x.clone(), y.clone()) == Some(start))
        .map(|(_, v)| v.clone())
        .min().unwrap()
}

fn find_shortest_path_lengths(grid: &Grid<char>) -> HashMap<(i32, i32), i32> {
    let edges: Vec<((i32, i32), (i32, i32))> = grid.edges().iter()
        .filter_map(|((vx1, c1), (vx2, c2))| {
            if valid(c1, c2) { Some((vx2.clone(), vx1.clone())) } else { None }
        })
        .collect_vec();


    let g = DiGraphMap::<(i32, i32), ()>::from_edges(edges);

    let end = grid.location_of(&'E').unwrap();

    dijkstra(&g, end, None, |_| 1)
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
    }
}







