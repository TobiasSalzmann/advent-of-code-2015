use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use itertools::{Itertools};
use pathfinding::num_traits::Pow;
use pathfinding::prelude::dijkstra;
use crate::util;

pub fn main() {
    let input: Vec<Snafu> = util::parse_from_strings("resources/day25.txt");

    println!("Day 25, Part 1: {:?}", sum_snafus(input));
}

fn sum_snafus(snafus: Vec<Snafu>) -> String {
    let sum = snafus.iter()
        .map(|sn| sn.0)
        .sum();
    Snafu(sum).to_snafu_string()
}

struct Snafu(i128);

impl FromStr for Snafu {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Snafu(s.chars()
            .rev()
            .enumerate()
            .map(|(i, c)| {
                let x = match c {
                    '=' => -2,
                    '-' => -1,
                    _ => c.to_digit(10).unwrap() as i128
                };
                x * 5i128.pow(i as u32)
            })
            .sum()))
    }
}

impl Snafu {
    fn to_snafu_string(&self) -> String {
        let digits = (0..)
            .map(|d| self.digit(d))
            .take_while(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect_vec();
        digits.iter().rev().collect()
    }

    fn digit(&self, i: u32) -> Option<char> {
        if i == 0 {
            return match self.0 % 5 {
                0 => Some('0'),
                1 => Some('1'),
                2 => Some('2'),
                3 => Some('='),
                4 => Some('-'),
                _ => panic!()
            };
        }

        let x = 5i128.pow(i);
        let x_over_two = (x / 2) + 1;
        let j = ((self.0 - x_over_two) / x) % 5;
        if self.0 < x_over_two {
            return None;
        }
        return match j {
            0 => Some('1'),
            1 => Some('2'),
            2 => Some('='),
            3 => Some('-'),
            4 => Some('0'),
            x => panic!("{} {} {}", self.0, i, x)
        };
    }
}


#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use crate::day25::{Snafu, sum_snafus};

    #[test]
    fn should_get_digit_0() {
        //assert_eq!(Snafu(0).digit(0), Some('0'));
        assert_eq!(Snafu(1).digit(0), Some('1'));
        assert_eq!(Snafu(2).digit(0), Some('2'));
        assert_eq!(Snafu(3).digit(0), Some('='));
        assert_eq!(Snafu(4).digit(0), Some('-'));
        assert_eq!(Snafu(5).digit(0), Some('0'));
        assert_eq!(Snafu(6).digit(0), Some('1'));
    }

    #[test]
    fn should_get_digit_1() {
        let digit = 1;
        assert_eq!(Snafu(0).digit(digit), None);
        assert_eq!(Snafu(1).digit(digit), None);
        assert_eq!(Snafu(2).digit(digit), None);
        assert_eq!(Snafu(3).digit(digit), Some('1'));
        assert_eq!(Snafu(4).digit(digit), Some('1'));
        assert_eq!(Snafu(5).digit(digit), Some('1'));
        assert_eq!(Snafu(6).digit(digit), Some('1'));
        assert_eq!(Snafu(7).digit(digit), Some('1'));
        assert_eq!(Snafu(8).digit(digit), Some('2'));
    }

    #[test]
    fn should_get_digit_2() {
        let digit = 2;
        assert_eq!(Snafu(0).digit(digit), None);
        assert_eq!(Snafu(12).digit(digit), None);
        assert_eq!(Snafu(13).digit(digit), Some('1'));
        assert_eq!(Snafu(37).digit(digit), Some('1'));
        assert_eq!(Snafu(38).digit(digit), Some('2'));
    }

    #[test]
    fn should_solve_example() {
        let input = vec![
            "1=-0-2".to_string(),
            "12111".to_string(),
            "2=0=".to_string(),
            "21".to_string(),
            "2=01".to_string(),
            "111".to_string(),
            "20012".to_string(),
            "112".to_string(),
            "1=-1=".to_string(),
            "1-12".to_string(),
            "12".to_string(),
            "1=".to_string(),
            "122".to_string(),
        ];
        let snafus = input.iter().map(|x| x.parse().unwrap()).collect_vec();
        assert_eq!(sum_snafus(snafus), "2=-1=0")
    }
}





