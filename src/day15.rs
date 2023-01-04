use std::cmp::{max};

use std::str::FromStr;
use itertools::{Itertools};

use crate::util;

pub fn main() {
    let input = util::parse_from_strings("resources/day15.txt");

    println!("Day 15 Part 1: {}", highest_scoring_cookie(&input));
    println!("Day 15 Part 2: {}", highest_scoring_cookie_with_calories(&input, 500));
}

fn highest_scoring_cookie(ingredients: &Vec<Ingredient>) -> i32 {
    splits(ingredients.len() as i32, 100).iter()
        .map(|split| score(split, ingredients))
        .max().unwrap()
}

fn highest_scoring_cookie_with_calories(ingredients: &Vec<Ingredient>, calories: i32) -> i32 {
    splits(ingredients.len() as i32, 100).iter()
        .filter(|split| count_calories(split, ingredients) == calories)
        .map(|split| score(split, ingredients))
        .max().unwrap()
}

fn score(split: &Vec<i32>, ingredients: &Vec<Ingredient>) -> i32 {
    let mut capacity = 0;
    let mut durability = 0;
    let mut flavor = 0;
    let mut texture = 0;

    for (amount, ingredient) in split.iter().zip_eq(ingredients.iter()) {
        capacity += amount * ingredient.capacity;
        durability += amount * ingredient.durability;
        flavor += amount * ingredient.flavor;
        texture += amount * ingredient.texture;
    }

    max(capacity, 0)
        * max(durability, 0)
        * max(flavor, 0)
        * max(texture, 0)
}

fn count_calories(split: &Vec<i32>, ingredients: &Vec<Ingredient>) -> i32 {
    let zipper = |(amount, ingredient)| amount * ingredient.calories;
    split.iter().zip_eq(ingredients.iter())
        .map(zipper)
        .sum()
}

fn splits(n: i32, target: i32) -> Vec<Vec<i32>> {
    if n == 1 {
        return vec![vec![target]];
    }
    let mut vecs = vec![];
    for i in 0..=target {
        for mut split in splits(n - 1, target - i) {
            split.push(i);
            vecs.push(split);
        }
    }
    vecs
}


#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Debug)]
struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl FromStr for Ingredient {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (capacity, durability, flavor, texture, calories) = s.split_whitespace()
            .filter_map(|s| s.trim_end_matches(',').parse::<i32>().ok())
            .collect_tuple().unwrap();
        Ok(Ingredient { capacity, durability, flavor, texture, calories })
    }
}