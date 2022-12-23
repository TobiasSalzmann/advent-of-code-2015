use std::collections::{HashMap, HashSet};
use itertools::{Itertools};
use crate::day22::Instruction::{Forward, Left, Right};
use crate::day22::Tile::{Empty, Rock};
use crate::util;

pub fn main() {
    let input: Vec<String> = util::parse_strings("resources/day22.txt");
    let (map, instructions) = parse(input);

    println!("Day 21, Part 1: {:?}", password(instructions.clone(), &map));
    println!("Day 22, Part 2: {:?}", cube_password(instructions, &map));
}

fn cube_password(instructions: Vec<Instruction>, map: &HashMap<(i32, i32), Tile>) -> i32 {
    let size = f64::sqrt((map.len() / 6) as f64) as i32;
    let mut edges = vec![];
    let mut corners = HashMap::new();
    for y in 0..6 {
        for x in 0..6 {
            let top_left = (x * size, y * size);
            let top_right = ((x + 1) * size, y * size);
            let bottom_left = (x * size, (y + 1) * size);
            let bottom_right = ((x + 1) * size, (y + 1) * size);
            if map.contains_key(&top_left) {
                let right = CubeEdge { start: top_right, end: bottom_right, side: 0 };
                let bottom = CubeEdge { start: bottom_left, end: bottom_right, side: 1 };
                let left = CubeEdge { start: top_left, end: bottom_left, side: 2 };
                let top = CubeEdge { start: top_left, end: top_right, side: 3 };
                for e in [right, bottom, left, top] {
                    corners.entry(vec![e.start.clone()]).or_insert(HashSet::new()).insert(e.clone());
                    corners.entry(vec![e.end.clone()]).or_insert(HashSet::new()).insert(e.clone());
                    edges.push(e)
                }
            }
        }
    }

    while let Some((c1, c2)) = find_mergable_corners(&corners, &edges) {
        let edges1 = corners.remove(&c1).unwrap();
        let edges2 = corners.remove(&c2).unwrap();
        let mut new_key = c1.clone();
        new_key.extend(c2);
        new_key.sort();
        corners.insert(new_key, edges1.union(&edges2).cloned().collect());
    }

    assert!(corners.len() == 8);

    // pair edges

    let edge_pairs: Vec<(CubeEdge, CubeEdge)> = edges
        .iter()
        .sorted_by_key(|e| get_corners(&corners, &(**e).clone()).iter().cloned().sorted().collect_vec())
        .group_by(|e| get_corners(&corners, &(**e).clone()).iter().cloned().sorted().collect_vec())
        .into_iter()
        .filter_map(|(_, v)| v.cloned().collect_tuple::<(CubeEdge, CubeEdge)>())
        .collect_vec();

    // for edge_pair in edge_pairs {
    //     println!("{:?}", get_corners(&corners, &edge_pair.0).iter().sorted().collect_vec());
    //     println!("{:?}", get_corners(&corners, &edge_pair.1).iter().sorted().collect_vec());
    //     println!("{:?}\n{:?}\n", edge_pair.0, edge_pair.1)
    // }

    let mut portals: HashMap<((i32, i32), i32), ((i32, i32), i32)> = HashMap::new();
    for (e1, e2) in edge_pairs.clone() {
        let corner = get_corners(&corners, &e1).iter().sorted().collect_vec()[0].clone();
        let a = corner.contains(&e1.start);
        let b = corner.contains(&e2.start);
        let is_opposite = a & !b || !a && b;
        if is_opposite {
            for (k, v) in e1.to_points().iter().zip(e2.to_points_incoming().iter().rev()) {
                portals.insert(k.clone(), v.clone());
            }
            for (k, v) in e2.to_points().iter().zip(e1.to_points_incoming().iter().rev()) {
                portals.insert(k.clone(), v.clone());
            }
        } else {
            for (k, v) in e1.to_points().iter().zip(e2.to_points_incoming().iter()) {
                portals.insert(k.clone(), v.clone());
            }
            for (k, v) in e2.to_points().iter().zip(e1.to_points_incoming().iter()) {
                portals.insert(k.clone(), v.clone());
            }
        }
    }

    cube_password_with_portals(instructions, map, &portals)
}

fn cube_password_with_portals(instructions: Vec<Instruction>, map: &HashMap<(i32, i32), Tile>, portals: &HashMap<((i32, i32), i32), ((i32, i32), i32)>) -> i32 {
    let (mut at_x, mut at_y) = initial_position(map);
    let mut facing = 0;
    for instruction in instructions {
        match instruction {
            Forward(n) => for _ in 0..n {
                let try_next = match facing {
                    0 => (at_x + 1, at_y),
                    1 => (at_x, at_y + 1),
                    2 => (at_x - 1, at_y),
                    3 => (at_x, at_y - 1),
                    _ => panic!()
                };
                let ((new_x, new_y), new_face) = match map.get(&try_next) {
                    Some(_) => (try_next, facing),
                    None => portals.get(&((at_x, at_y), facing)).unwrap().clone()
                };
                if let Some(Empty) = map.get(&(new_x, new_y)) {
                    at_x = new_x;
                    at_y = new_y;
                    facing = new_face;
                }
            },
            Left => { facing = (facing - 1 + 4) % 4; }
            Right => { facing = (facing + 1) % 4; }
        }
    }

    1000 * (at_y + 1) + 4 * (at_x + 1) + facing
}

fn find_mergable_corners(corners: &HashMap<Vec<(i32, i32)>, HashSet<CubeEdge>>, edges: &Vec<CubeEdge>) -> Option<(Vec<(i32, i32)>, Vec<(i32, i32)>)> {
    let outside_edges: Vec<CubeEdge> = edges
        .iter()
        .sorted_by_key(|e| get_corners(corners, &(**e).clone()).iter().cloned().sorted().collect_vec())
        .group_by(|e| get_corners(corners, &(**e).clone()).iter().cloned().sorted().collect_vec())
        .into_iter()
        .flat_map(|(_, v)| {
            let vec = v.collect_vec();
            if vec.len() == 1 { vec } else { vec![] }
        })
        .cloned()
        .collect_vec();

    for e1 in &outside_edges {
        for e2 in &outside_edges {
            let e1_corners: HashSet<Vec<(i32, i32)>> = get_corners(corners, e1);
            let e2_corners: HashSet<Vec<(i32, i32)>> = get_corners(corners, e2);
            let intersection: Vec<&Vec<(i32, i32)>> = e1_corners.intersection(&e2_corners)
                .collect_vec();
            if intersection.len() == 1 && corners.get(intersection[0]).unwrap().len() == 6 {
                return e1_corners.symmetric_difference(&e2_corners)
                    .cloned()
                    .collect_tuple();
            }
        }
    }
    None
}

fn get_corners(corners: &HashMap<Vec<(i32, i32)>, HashSet<CubeEdge>>, e1: &CubeEdge) -> HashSet<Vec<(i32, i32)>> {
    corners.iter()
        .filter(|(_, v)| v.contains(e1))
        .map(|(k, _)| k.clone())
        .collect()
}

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
struct CubeEdge {
    start: (i32, i32),
    end: (i32, i32),
    side: i32,
}

impl CubeEdge {
    fn to_points(&self) -> Vec<((i32, i32), i32)> {
        match self.side.clone() {
            0 => (self.start.1..self.end.1).map(|y| ((self.start.0 - 1, y), 0)).collect_vec(),
            1 => (self.start.0..self.end.0).map(|x| ((x, self.start.1 - 1), 1)).collect_vec(),
            2 => (self.start.1..self.end.1).map(|y| ((self.start.0, y), 2)).collect_vec(),
            3 => (self.start.0..self.end.0).map(|x| ((x, self.start.1), 3)).collect_vec(),
            _ => panic!()
        }
    }

    fn to_points_incoming(&self) -> Vec<((i32, i32), i32)> {
        self.to_points().iter()
            .map(|(p, d)| (p.clone(), (d.clone() + 2) % 4)).collect_vec()
    }
}


fn password(instructions: Vec<Instruction>, map: &HashMap<(i32, i32), Tile>) -> i32 {
    let (mut at_x, mut at_y) = initial_position(map);
    let mut facing = 0;
    for instruction in instructions {
        match instruction {
            Forward(n) => for _ in 0..n {
                if let (new_x, new_y, Empty) = next_tile(map, at_x, at_y, &mut facing) {
                    at_x = new_x;
                    at_y = new_y;
                };
            },
            Left => { facing = (facing - 1 + 4) % 4; }
            Right => { facing = (facing + 1) % 4; }
        }
    }

    1000 * (at_y + 1) + 4 * (at_x + 1) + facing
}

fn next_tile(map: &HashMap<(i32, i32), Tile>, at_x: i32, at_y: i32, facing: &mut i32) -> (i32, i32, Tile) {
    let ((new_x, new_y), new_t) = match facing {
        0 => right(map, at_x, at_y),
        1 => down(map, at_x, at_y),
        2 => left(map, at_x, at_y),
        3 => up(map, at_x, at_y),
        _ => panic!()
    };
    (new_x, new_y, new_t)
}

fn right(map: &HashMap<(i32, i32), Tile>, at_x: i32, at_y: i32) -> ((i32, i32), Tile) {
    let new_pos = (at_x + 1, at_y);
    match map.get(&new_pos) {
        Some(t) => (new_pos, t.clone()),
        None => {
            let (p, t) = map.iter()
                .filter(|((_, y), _)| *y == at_y)
                .min().unwrap();
            (p.clone(), t.clone())
        }
    }
}

fn left(map: &HashMap<(i32, i32), Tile>, at_x: i32, at_y: i32) -> ((i32, i32), Tile) {
    let new_pos = (at_x - 1, at_y);
    match map.get(&new_pos) {
        Some(t) => (new_pos, t.clone()),
        None => {
            let (p, t) = map.iter()
                .filter(|((_, y), _)| *y == at_y)
                .max().unwrap();
            (p.clone(), t.clone())
        }
    }
}

fn down(map: &HashMap<(i32, i32), Tile>, at_x: i32, at_y: i32) -> ((i32, i32), Tile) {
    let new_pos = (at_x, at_y + 1);
    match map.get(&new_pos) {
        Some(t) => (new_pos, t.clone()),
        None => {
            let (p, t) = map.iter()
                .filter(|((x, _), _)| *x == at_x)
                .min().unwrap();
            (p.clone(), t.clone())
        }
    }
}

fn up(map: &HashMap<(i32, i32), Tile>, at_x: i32, at_y: i32) -> ((i32, i32), Tile) {
    let new_pos = (at_x, at_y - 1);
    match map.get(&new_pos) {
        Some(t) => (new_pos, t.clone()),
        None => {
            let (p, t) = map.iter()
                .filter(|((x, _), _)| *x == at_x)
                .max().unwrap();
            (p.clone(), t.clone())
        }
    }
}

fn initial_position(map: &HashMap<(i32, i32), Tile>) -> (i32, i32) {
    map.iter()
        .filter(|((_, y), t)| *y == 0 && **t == Empty)
        .map(|(p, _)| p)
        .min()
        .unwrap()
        .clone()
}

fn parse(lines: Vec<String>) -> (HashMap<(i32, i32), Tile>, Vec<Instruction>) {
    let grid: HashMap<(i32, i32), Tile> = lines[0..lines.len() - 2]
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line
                .chars()
                .enumerate()
                .filter(|(_, c)| c != &' ')
                .map(move |(x, c)| ((x as i32, y as i32), c.into()))
        }).collect();
    let instructions = parse_instructions(&lines[lines.len() - 1]);
    (grid, instructions)
}

fn parse_instructions(line: &String) -> Vec<Instruction> {
    line
        .replace("R", " R ")
        .replace("L", " L ")
        .split_whitespace()
        .map(Instruction::from)
        .collect()
}

#[derive(Debug, PartialEq, Clone, Ord, PartialOrd, Eq)]
enum Tile {
    Rock,
    Empty,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '#' => Rock,
            '.' => Empty,
            _ => panic!(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Instruction {
    Forward(i32),
    Left,
    Right,
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        match s {
            "L" => Left,
            "R" => Right,
            other => Forward(other.parse().unwrap()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::day22::Instruction::{Forward, Left, Right};
    use crate::day22::{cube_password, parse, password};
    use crate::day22::Tile::Rock;

    #[test]
    fn should_solve_example() {
        let input = vec![
            "        ...#".to_string(),
            "        .#..".to_string(),
            "        #...".to_string(),
            "        ....".to_string(),
            "...#.......#".to_string(),
            "........#...".to_string(),
            "..#....#....".to_string(),
            "..........#.".to_string(),
            "        ...#....".to_string(),
            "        .....#..".to_string(),
            "        .#......".to_string(),
            "        ......#.".to_string(),
            "".to_string(),
            "10R5L5R10L4R5L5".to_string(),
        ];
        let (map, is) = parse(input);
        assert_eq!(map.len(), 96);
        assert_eq!(map.iter().filter(|(_, x)| **x == Rock).count(), 13);
        assert_eq!(is, vec![
            Forward(10),
            Right,
            Forward(5),
            Left,
            Forward(5),
            Right,
            Forward(10),
            Left,
            Forward(4),
            Right,
            Forward(5),
            Left,
            Forward(5),
        ]);

        assert_eq!(password(is, &map), 6032)
    }

    #[test]
    fn should_solve_example_2() {
        let input = vec![
            "        ...#".to_string(),
            "        .#..".to_string(),
            "        #...".to_string(),
            "        ....".to_string(),
            "...#.......#".to_string(),
            "........#...".to_string(),
            "..#....#....".to_string(),
            "..........#.".to_string(),
            "        ...#....".to_string(),
            "        .....#..".to_string(),
            "        .#......".to_string(),
            "        ......#.".to_string(),
            "".to_string(),
            "10R5L5R10L4R5L5".to_string(),
        ];
        let (map, is) = parse(input);

        assert_eq!(cube_password(is, &map), 5031)
    }
}
