use crate::util;

pub fn main() {
    let input = util::parse_strings("resources/day1.txt").first().unwrap().clone();

    println!("Day 1, Part 1: {:?}", final_floor(&input));
    println!("Day 1, Part 2: {:?}", basement_index(&input));
    // println!("Day 1, Part 2: {:?} calories", max_3_calories(input));
}

fn final_floor(instructions: &String) -> u32 {
    let mut floor = 0;
    for c in instructions.chars() {
        match c {
            '(' => floor += 1,
            _ => floor -= 1,
        }
    }
    floor
}

fn basement_index(instructions: &String) -> usize {
    let mut floor = 0;
    for (i, c) in instructions.chars().enumerate() {
        match c {
            '(' => floor += 1,
            _ => floor -= 1,
        }
        if floor == -1 {
            return i + 1;
        }
    }
    panic!()
}