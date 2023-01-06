use itertools::Itertools;

use crate::util;

pub fn main() {
    let input = util::parse_strings("resources/day19.txt");
    let (transformations, molecule) = parse(&input);


    println!("Day 19 Part 1: {}", count_replacements(&transformations, &molecule));
}

fn count_replacements(transformations: &Vec<(String, Vec<String>)>, molecule: &Vec<String>) -> usize {
    let mut molecules = vec![];
    for (target, replacement) in transformations {
        let indexes = molecule.iter().positions(|c| c == target).collect_vec();
        for index in indexes {
            let mut new_molecule = molecule.clone();
            new_molecule.splice(index..=index, replacement.clone()).count();
            molecules.push(new_molecule);
        }
    }
    let molecules = molecules.iter().unique().collect_vec();
    molecules.len()
}

fn parse(input: &Vec<String>) -> (Vec<(String, Vec<String>)>, Vec<String>) {
    let transformations: Vec<(String, Vec<String>)> = input.iter()
        .take_while(|line| !line.is_empty())
        .map(|line| line.split(" => ").collect_tuple::<(&str, &str)>().unwrap())
        .map(|(a, b)| (a.to_string(), separate(b.to_string())))
        .collect();
    let molecule = separate(input.last().unwrap().clone());
    (transformations, molecule)
}

fn separate(molecule: String) -> Vec<String> {
    molecule.chars()
        .flat_map(|c| {
            if c.is_ascii_uppercase() { vec![' ', c] } else { vec![c] }
        })
        .collect::<String>()
        .trim()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect_vec()
}
