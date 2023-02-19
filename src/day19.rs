use std::collections::HashMap;
use itertools::Itertools;

use crate::util;

pub fn main() {
    let input = util::parse_strings("resources/day19.txt");
    let (transformations, molecule) = parse(&input);


    println!("Day 19 Part 1: {}", count_replacements(&transformations, &molecule));
    println!("Day 19 Part 2: {}", cheapest_match(&transformations, &molecule))
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

#[derive(Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Debug)]
struct Match {
    start: usize,
    end: usize,
    atom: String,
}

type Atom = String;
type Molecule = Vec<Atom>;

fn cheapest_match(transformations: &Vec<(Atom, Molecule)>, molecule: &Molecule) -> usize {
    let mut matches: HashMap<Match, usize> = molecule.iter()
        .enumerate()
        .map(|(i, s)| (Match {
            start: i,
            end: i+1,
            atom: s.clone(),
        }, 0)).collect();
    for match_length in 2..=molecule.len() {
        let mut number_of_matches = matches.len();
        loop {
            for start in 0..molecule.len()-match_length+1 {
                let end = start + match_length;
                for (atom, seq) in transformations {
                    if let Some(cheapest_split) = get_cheapest_split(start, end,seq.as_slice(), &matches) {
                        let m = Match {
                            start,
                            end,
                            atom: atom.clone(),
                        };
                        matches.insert(m, cheapest_split + 1);
                    }
                }
            }
            if number_of_matches == matches.len() {
                break
            } else {
                number_of_matches = matches.len()
            }
        }
    }

    matches.get(&Match{
        start: 0,
        end: molecule.len(),
        atom: "e".to_string(),
    }).unwrap().clone()
}

fn get_cheapest_split(start: usize, end: usize, seq: &[Atom], costs: &HashMap<Match, usize>) -> Option<usize> {
    if seq.is_empty() && start == end {
        return Some(0);
    }
    if seq.is_empty() {
        return None;
    }
    if start >= end {
        return None
    }
    let mut min: Option<usize> = None;
    for first_end in start+1..=end {
        let m = Match {
            start,
            end: first_end,
            atom: seq[0].clone(),
        };
        if let Some(first_costs) = costs.get(&m) {
            if let Some(c) =  get_cheapest_split(first_end, end, &seq[1..], costs) {
                let total_costs = first_costs + c;
                if min.is_none() || min.unwrap() > total_costs {
                    min = Some(total_costs)
                }
            }
        }
    }
    min
}
