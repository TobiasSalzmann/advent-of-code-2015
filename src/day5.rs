use itertools::Itertools;
use crate::day5::CrateMoverVersion::{V9000, V9001};
use crate::util;

pub fn main() {
    let input = util::parse_strings("resources/day5.txt");
    let (stacks, moves) = parse_cargo(input);

    println!("Day 5, Part 1: Top crates: {:?}", top_crates(stacks.clone(), moves.clone(), V9000));
    println!("Day 5, Part 2: Top crates: {:?}", top_crates(stacks, moves, V9001));
}

fn parse_cargo(raw_cargo: Vec<String>) -> (Vec<Vec<char>>, Vec<Move>) {
    let (raw_stacks, raw_moves): (&[String], &[String]) = raw_cargo
        .split(|line| line.is_empty())
        .collect_tuple()
        .unwrap();

    let stacks = parse_stacks(raw_stacks);
    let moves = parse_moves(raw_moves);

    (stacks, moves)
}

fn parse_moves(raw_moves: &[String]) -> Vec<Move> {
    raw_moves.iter()
        .map(parse_move)
        .collect_vec()
}

fn parse_move(raw_move: &String) -> Move {
    match &raw_move.split(|c: char| !c.is_numeric()).filter(|s| !s.is_empty()).collect_vec()[..] {
        [count, from, to] => Move {
            count: count.parse().unwrap(),
            from: from.parse().unwrap(),
            to: to.parse().unwrap(),
        },
        x => panic!("{:?}", x)
    }
}

fn parse_stacks(raw_stacks: &[String]) -> Vec<Vec<char>> {
    let rows = raw_stacks.iter()
        .map(|s| s.chars().collect_vec())
        .collect_vec();
    let number_of_stacks = count_stacks(&rows);
    let mut stacks = vec![Vec::new()];

    for col_idx in 0..number_of_stacks {
        let mut stack = Vec::new();
        let tallest_stack_height = rows.len() - 1;
        for row_idx in (0..tallest_stack_height).rev() {
            if let Some(c) = rows[row_idx].get(1 + col_idx * 4) {
                if !c.is_whitespace() {
                    stack.push(*c);
                }
            }
        }
        stacks.push(stack)
    }
    stacks
}

fn count_stacks(rows: &Vec<Vec<char>>) -> usize {
    rows.last()
        .unwrap()
        .iter()
        .filter(|s| !s.is_whitespace())
        .count()
}

fn top_crates(mut stacks: Vec<Vec<char>>, moves: Vec<Move>, crate_mover_version: CrateMoverVersion) -> String {
    apply_moves(&mut stacks, moves, crate_mover_version);
    stacks.iter()
        .filter(|stack| !stack.is_empty())
        .map(|stack| stack.last().unwrap())
        .collect()
}

fn apply_moves(stacks: &mut Vec<Vec<char>>, moves: Vec<Move>, crate_mover_version: CrateMoverVersion) {
    for Move { count, from, to } in moves {
        let n = stacks[from].len();
        let mut crates_to_move: Vec<char> = stacks[from].split_off(n - count as usize);
        if crate_mover_version == V9000 {
            crates_to_move.reverse();
        }

        stacks[to].append(&mut crates_to_move);
    }
}

#[derive(PartialEq)]
enum CrateMoverVersion {
    V9000,
    V9001
}


#[derive(Eq, PartialEq, Debug, Clone)]
struct Move {
    count: i32,
    from: usize,
    to: usize,
}

#[cfg(test)]
mod tests {
    use crate::day5::*;

    #[test]
    fn parse_example() {
        let input = vec![
            "    [D]".to_string(),
            "[N] [C]".to_string(),
            "[Z] [M] [P]".to_string(),
            " 1   2   3".to_string(),
            "".to_string(),
            "move 1 from 2 to 1".to_string(),
            "move 3 from 1 to 3".to_string(),
            "move 2 from 2 to 1".to_string(),
            "move 1 from 1 to 2".to_string(),
        ];
        let stacks = vec![
            vec![],
            vec!['Z', 'N'],
            vec!['M', 'C', 'D'],
            vec!['P'],
        ];
        let moves = vec![
            Move { count: 1, from: 2, to: 1 },
            Move { count: 3, from: 1, to: 3 },
            Move { count: 2, from: 2, to: 1 },
            Move { count: 1, from: 1, to: 2 },
        ];
        assert_eq!(parse_cargo(input), (stacks, moves));
    }

    #[test]
    fn finds_top_crates_after_rearranging() {
        let stacks = vec![
            vec![],
            vec!['Z', 'N'],
            vec!['M', 'C', 'D'],
            vec!['P'],
        ];
        let moves = vec![
            Move { count: 1, from: 2, to: 1 },
            Move { count: 3, from: 1, to: 3 },
            Move { count: 2, from: 2, to: 1 },
            Move { count: 1, from: 1, to: 2 },
        ];

        assert_eq!(top_crates(stacks, moves, V9000), "CMZ");
    }

    #[test]
    fn finds_top_crates_after_rearranging_with_crate_mover9001() {
        let stacks = vec![
            vec![],
            vec!['Z', 'N'],
            vec!['M', 'C', 'D'],
            vec!['P'],
        ];
        let moves = vec![
            Move { count: 1, from: 2, to: 1 },
            Move { count: 3, from: 1, to: 3 },
            Move { count: 2, from: 2, to: 1 },
            Move { count: 1, from: 1, to: 2 },
        ];

        assert_eq!(top_crates(stacks, moves, V9001), "MCD");
    }
}







