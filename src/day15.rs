use std::cmp::Ordering;
use std::str::FromStr;
use itertools::Itertools;
use crate::day15::ExtremePoint::{End, Start};

use crate::util;

pub fn main() {
    let input: Vec<Sensor> = util::parse_from_strings("resources/day15.txt");

    println!(
        "Day 15, Part 1: {:?}", impossible_beacons(input.clone(), 2000000)
    );
}

fn impossible_beacons(sensors: Vec<Sensor>, row: i32) -> i32 {
    let disjoint_intervals = disjoint_intervals(&sensors, row);
    let interval_length_sum = total_interval_length(disjoint_intervals);
    let beacons_in_row = beacons_in_row(sensors, row);
    interval_length_sum - beacons_in_row
}

fn total_interval_length(disjoint_intervals: Vec<(i32, i32)>) -> i32 {
    disjoint_intervals.iter()
        .map(|(start, end)| end - start + 1)
        .sum()
}

fn disjoint_intervals(sensors: &Vec<Sensor>, row: i32) -> Vec<(i32, i32)> {
    let extreme_points = sensor_breadth_points(&sensors, row);
    let mut depth = 0;
    let mut current_start = -1;
    let mut disjoint_intervals: Vec<(i32, i32)> = vec![];
    for point in extreme_points {
        match point {
            Start(start_x) => {
                if depth == 0 { current_start = start_x };
                depth += 1
            }
            End(end_x) => {
                depth -= 1;
                if depth == 0 { disjoint_intervals.push((current_start, end_x)) }
            }
        }
    }
    disjoint_intervals
}

fn sensor_breadth_points(sensors: &Vec<Sensor>, row: i32) -> Vec<ExtremePoint> {
    sensors.iter()
        .flat_map(|sensor| {
            if let Some((start, end)) = sensor.overlap_with_row(row) {
                vec![Start(start), End(end)]
            } else {
                vec![]
            }
        })
        .sorted()
        .collect_vec()
}

fn beacons_in_row(sensors: Vec<Sensor>, row: i32) -> i32 {
    sensors.iter()
        .filter(|sensor| sensor.closest_beacon_location.1 == row)
        .map(|sensor| sensor.closest_beacon_location.0)
        .unique()
        .count() as i32
}

#[derive(Eq, PartialEq, Debug)]
enum ExtremePoint {
    Start(i32),
    End(i32),
}

impl ExtremePoint {
    fn inner(&self) -> i32 {
        match self {
            Start(x) => x.clone(),
            End(x) => x.clone()
        }
    }
}

impl Ord for ExtremePoint {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Start(a), End(b)) if a == b => Ordering::Less,
            (End(a), Start(b)) if a == b => Ordering::Greater,
            (x, y) => x.inner().cmp(&y.inner())
        }
    }
}

impl PartialOrd<Self> for ExtremePoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone)]
struct Sensor {
    location: (i32, i32),
    closest_beacon_location: (i32, i32),
}

impl Sensor {
    fn distance_to_beacon(&self) -> i32 {
        let dx = (self.location.0 - self.closest_beacon_location.0).abs();
        let dy = (self.location.1 - self.closest_beacon_location.1).abs();
        dx + dy
    }

    fn overlap_with_row(&self, row: i32) -> Option<(i32, i32)> {
        let distance = self.distance_to_beacon();
        let distance_to_row = (self.location.1 - row).abs();
        let remaining_distance = distance - distance_to_row;
        if remaining_distance < 0 { return None; }
        Some((self.location.0 - remaining_distance, self.location.0 + remaining_distance))
    }
}


impl FromStr for Sensor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (sensor_x, sensor_y, beacon_x, beacon_y) = s
            .split(|c: char| !c.is_numeric() && !(c == '-'))
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<i32>().unwrap())
            .collect_tuple().unwrap();

        Ok(Sensor {
            location: (sensor_x, sensor_y),
            closest_beacon_location: (beacon_x, beacon_y),
        })
    }
}


#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use crate::day15::{impossible_beacons, Sensor};

    #[test]
    fn should_do_stuff() {
        let input: Vec<Sensor> = vec![
            "Sensor at x=2, y=18: closest beacon is at x=-2, y=15".to_string(),
            "Sensor at x=9, y=16: closest beacon is at x=10, y=16".to_string(),
            "Sensor at x=13, y=2: closest beacon is at x=15, y=3".to_string(),
            "Sensor at x=12, y=14: closest beacon is at x=10, y=16".to_string(),
            "Sensor at x=10, y=20: closest beacon is at x=10, y=16".to_string(),
            "Sensor at x=14, y=17: closest beacon is at x=10, y=16".to_string(),
            "Sensor at x=8, y=7: closest beacon is at x=2, y=10".to_string(),
            "Sensor at x=2, y=0: closest beacon is at x=2, y=10".to_string(),
            "Sensor at x=0, y=11: closest beacon is at x=2, y=10".to_string(),
            "Sensor at x=20, y=14: closest beacon is at x=25, y=17".to_string(),
            "Sensor at x=17, y=20: closest beacon is at x=21, y=22".to_string(),
            "Sensor at x=16, y=7: closest beacon is at x=15, y=3".to_string(),
            "Sensor at x=14, y=3: closest beacon is at x=15, y=3".to_string(),
            "Sensor at x=20, y=1: closest beacon is at x=15, y=3".to_string(),
        ].iter()
            .map(|s| s.parse().unwrap())
            .collect_vec();

        assert_eq!(impossible_beacons(input, 10), 26);
    }
}
