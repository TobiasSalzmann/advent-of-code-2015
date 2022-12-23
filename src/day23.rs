use std::collections::{HashSet, VecDeque};
use itertools::{Itertools};
use crate::day23::Direction::{East, North, South, West};
use crate::util;

pub fn main() {
    let input: Vec<String> = util::parse_strings("resources/day23.txt");
    let initial_elves = parse(input);

    println!("Day 23, Part 1: {:?}", count_empty_after_rounds(initial_elves.clone(), 10));
    println!("Day 23, Part 2: {:?}", first_stable_round(initial_elves));
}

fn parse(raw: Vec<String>) -> HashSet<Elf> {
    let mut elves = HashSet::new();
    for (y, line) in raw.into_iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                elves.insert(Elf { x: x as i32, y: y as i32 });
            }
        }
    }
    elves
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Clone)]
struct Elf {
    x: i32,
    y: i32,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

fn count_empty_after_rounds(mut elves: HashSet<Elf>, rounds: usize) -> i32 {
    let mut move_priorities = VecDeque::from([North, South, West, East]);
    for _ in 0..rounds {
        elves = simulate_round(elves, &move_priorities);

        let first = move_priorities.pop_front().unwrap();
        move_priorities.push_back(first);
    }

    count_empty(&elves)
}

fn first_stable_round(mut elves: HashSet<Elf>) -> i32 {
    let mut move_priorities = VecDeque::from([North, South, West, East]);
    for i in 0.. {
        let old_elves = elves.clone();
        let new_elves = simulate_round(elves, &move_priorities);
        if new_elves == old_elves {
            return i + 1;
        }
        elves = new_elves;


        let first = move_priorities.pop_front().unwrap();
        move_priorities.push_back(first);
    }
    return -1
}

fn count_empty(elves: &HashSet<Elf>) -> i32 {
    let min_x = elves.iter().map(|e| e.x).min().unwrap();
    let max_x = elves.iter().map(|e| e.x).max().unwrap();
    let min_y = elves.iter().map(|e| e.y).min().unwrap();
    let max_y = elves.iter().map(|e| e.y).max().unwrap();
    (max_x - min_x + 1) * (max_y - min_y + 1) - (elves.len() as i32)
}

fn simulate_round(elves: HashSet<Elf>, move_priorities: &VecDeque<Direction>) -> HashSet<Elf> {
    let elves_by_proposal = elves.iter()
        .map(|elf| (proposal(elf, move_priorities, &elves), elf.clone()))
        .into_group_map();
    elves_by_proposal.into_iter()
        .flat_map(|(proposal, elves)| {
            if elves.len() == 1 {
                vec![proposal]
            } else {
                elves
            }
        }).collect()
}

fn proposal(elf: &Elf, priorities: &VecDeque<Direction>, elves: &HashSet<Elf>) -> Elf {
    let Elf { x, y } = elf.clone();
    let free = |x, y| { elves.get(&Elf { x, y }).is_none() };
    let nw_free = free(x - 1, y - 1);
    let n_free = free(x, y - 1);
    let ne_free = free(x + 1, y - 1);
    let sw_free = free(x - 1, y + 1);
    let w_free = free(x - 1, y);
    let e_free = free(x + 1, y);
    let se_free = free(x + 1, y + 1);
    let s_free = free(x, y + 1);
    if nw_free && n_free && ne_free && e_free && se_free && s_free && sw_free && w_free {
        return elf.clone()
    }
    for prio in priorities {
        match prio {
            North if nw_free && n_free && ne_free => return Elf { x, y: y - 1 },
            South if sw_free && s_free && se_free => return Elf { x, y: y + 1 },
            West if nw_free && w_free && sw_free => return Elf { x: x - 1, y },
            East if ne_free && e_free && se_free => return Elf { x: x + 1, y },
            _ => {}
        }
    }
    return elf.clone();
}

#[cfg(test)]
mod tests {
    use crate::day23::{count_empty_after_rounds, first_stable_round, parse};

    #[test]
    fn should_solve_example() {
        let input = vec![
            "....#..".to_string(),
            "..###.#".to_string(),
            "#...#.#".to_string(),
            ".#...##".to_string(),
            "#.###..".to_string(),
            "##.#.##".to_string(),
            ".#..#..".to_string(),
        ];
        let empties = count_empty_after_rounds(parse(input), 10);
        assert_eq!(empties, 110)
    }

    #[test]
    fn should_solve_example_part2() {
        let input = vec![
            "....#..".to_string(),
            "..###.#".to_string(),
            "#...#.#".to_string(),
            ".#...##".to_string(),
            "#.###..".to_string(),
            "##.#.##".to_string(),
            ".#..#..".to_string(),
        ];
        let round = first_stable_round(parse(input));
        assert_eq!(round, 20)
    }


}

