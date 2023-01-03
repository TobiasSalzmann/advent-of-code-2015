use std::cmp::{min};

use std::str::FromStr;
use itertools::{Itertools};

use crate::util;

pub fn main() {
    let input = util::parse_from_strings("resources/day14.txt");

    println!("Day 14 Part 1: {}", winning_reindeer_after(&input, 2503));
    println!("Day 14 Part 2: {}", winning_reindeer_after_2(&input, 2503));
}

fn winning_reindeer_after(reindeers: &Vec<Reindeer>, total_time: u32) -> u32 {
    reindeers
        .iter()
        .map(|r| r.distance_after(total_time))
        .max().unwrap()
}

fn winning_reindeer_after_2(reindeers: &Vec<Reindeer>, total_time: u32) -> u32 {
    let mut points = reindeers.iter().map(|_| 0u32).collect_vec();
    for t in 1..=total_time {
        let distances = reindeers
            .iter()
            .map(|r| r.distance_after(t))
            .collect_vec();
        let max = distances.iter().max().unwrap();
        distances
            .iter()
            .positions(|d| d == max)
            .for_each(|idx| points[idx] += 1);
    }
    points.iter().max().unwrap().clone()
}


#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Debug)]
struct Reindeer {
    speed: u32,
    endurance: u32,
    rest_time: u32,
}

impl Reindeer {
    fn distance_after(&self, time: u32) -> u32 {
        let cycle_length = self.endurance + self.rest_time;
        let full_cycles = time / cycle_length;
        let remaining_active_time = min(time % cycle_length, self.endurance);
        let active_time = (full_cycles * self.endurance) + remaining_active_time;
        active_time * self.speed
    }
}

impl FromStr for Reindeer {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (speed, endurance, rest_time) = s.split_whitespace()
            .filter_map(|s| s.parse::<u32>().ok())
            .collect_tuple().unwrap();
        Ok(Reindeer { speed, endurance, rest_time })
    }
}