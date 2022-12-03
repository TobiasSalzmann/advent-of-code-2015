use itertools::Itertools;
use crate::util;

pub fn main() {
    let input = util::parse_strings("resources/day3.txt");

    println!("Day 3, Part 1: {:?} summed priorities", calculate_priority_sum(input.clone()));
    println!("Day 3, Part 2: {:?} summed badge priorities", badge_priority_sum(input));
}

fn calculate_priority_sum(backpacks: Vec<String>) -> i32 {
    backpacks
        .iter()
        .map(|backpack| calculate_priority(backpack))
        .sum()
}

fn calculate_priority(backpack: &String) -> i32 {
    let (front, back) = backpack.split_at(backpack.len() / 2);
    let duplicate_item = front.chars()
        .find(|c| back.contains(c.clone()))
        .expect("Expected there to be a duplicate item");
    priority(duplicate_item)
}

fn priority(item: char) -> i32 {
    if item.is_uppercase() {
        item as i32 - 'A' as i32 + 27
    } else {
        item as i32 - 'a' as i32 + 1
    }
}

fn badge_priority_sum(backpacks: Vec<String>) -> i32 {
    backpacks
        .chunks(3)
        .map(|group| badge_priority(group))
        .sum()
}

fn badge_priority(group: &[String]) -> i32 {
    let (first, second, third) = group.iter()
        .collect_tuple()
        .expect("Group should have exactly 3 members");
    let badge = common_characters(&common_characters(first, second), third)
        .chars()
        .next().expect("There should be a badge");
    priority(badge)
}

fn common_characters(first: &String, second: &String) -> String {
    first.chars()
        .filter(|c| second.contains(c.clone()))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::day3::*;

    #[test]
    fn calculates_priorities_for_example() {
        let input = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
            "PmmdzqPrVvPwwTWBwg".to_string(),
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string(),
            "ttgJtRGJQctTZtZT".to_string(),
            "CrZsJsPPZsGzwwsLwLmpwMDw".to_string(),
        ];
        assert_eq!(calculate_priority_sum(input), 157);
    }

    #[test]
    fn sums_badge_priorities_for_example() {
        let input = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
            "PmmdzqPrVvPwwTWBwg".to_string(),
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string(),
            "ttgJtRGJQctTZtZT".to_string(),
            "CrZsJsPPZsGzwwsLwLmpwMDw".to_string(),
        ];
        assert_eq!(badge_priority_sum(input), 70);
    }
}
