use std::cmp::Ordering;
use itertools::Itertools;
use crate::util;

pub fn main() {
    let input = util::parse_strings("resources/day4.txt");

    println!("Day 4, Part 1: {:?} encompassing tasks", count_encompassing_tasks(input.clone()));
    println!("Day 4, Part 2: {:?} overlapping tasks", count_overlapping_tasks(input.clone()));
}

fn count_encompassing_tasks(task_distribution: Vec<String>) -> usize {
    task_distribution.into_iter()
        .map(|raw| parse_task_distribution(&raw))
        .filter(|tasks| is_encompassing(tasks))
        .count()
}

fn is_encompassing(tasks: &TaskDistribution) -> bool {
    match tasks.astart.cmp(&tasks.bstart) {
        Ordering::Less => {tasks.aend >= tasks.bend}
        Ordering::Equal => {true}
        Ordering::Greater => {tasks.bend >= tasks.aend}
    }
}

fn count_overlapping_tasks(task_distribution: Vec<String>) -> usize {
    task_distribution.into_iter()
        .map(|raw| parse_task_distribution(&raw))
        .filter(|tasks| is_overlapping(tasks))
        .count()
}

fn is_overlapping(tasks: &TaskDistribution) -> bool {
    match tasks.astart.cmp(&tasks.bstart) {
        Ordering::Less => {tasks.bstart <= tasks.aend}
        Ordering::Equal => {true}
        Ordering::Greater => {tasks.astart <= tasks.bend}
    }
}

struct TaskDistribution {
    astart: i32,
    aend: i32,
    bstart: i32,
    bend: i32,
}

fn parse_task_distribution(tasks: &String) -> TaskDistribution {
    let (astart, aend, bstart, bend): (i32, i32, i32, i32) = tasks
        .split(|c| c == '-' || c == ',')
        .map(|s| s.parse().unwrap())
        .collect_tuple().unwrap();
    TaskDistribution{
        astart,
        aend,
        bstart,
        bend
    }
}

#[cfg(test)]
mod tests {
    use crate::day4::*;

    #[test]
    fn counts_encompassing_tasks_for_example() {
        let input = vec![
            "2-4,6-8".to_string(),
            "2-3,4-5".to_string(),
            "5-7,7-9".to_string(),
            "2-8,3-7".to_string(),
            "6-6,4-6".to_string(),
            "2-6,4-8".to_string(),
        ];
        assert_eq!(count_encompassing_tasks(input), 2);
    }

    #[test]
    fn counts_overlapping_tasks_for_example() {
        let input = vec![
            "2-4,6-8".to_string(),
            "2-3,4-5".to_string(),
            "5-7,7-9".to_string(),
            "2-8,3-7".to_string(),
            "6-6,4-6".to_string(),
            "2-6,4-8".to_string(),
        ];
        assert_eq!(count_overlapping_tasks(input), 4);
    }
}







