
use std::str::FromStr;
use itertools::{Itertools};
use pathfinding::prelude::astar;

use rayon::prelude::*;
use crate::util;

pub fn main() {
    let input: Vec<Blueprint> = util::parse_from_strings("resources/day19.txt");

    println!("Day 16, Part 1: {:?}", total_quality(input.clone()));
    println!("Day 16, Part 2: {:?}", geodes_product(input.clone()));
}

fn total_quality(blueprints: Vec<Blueprint>) -> usize {
    blueprints.par_iter()
        .map(|b| max_geodes(b, 24) * b.index)
        .sum()
}

fn geodes_product(blueprints: Vec<Blueprint>) -> usize {
    blueprints.par_iter()
        .take(3)
        .map(|b| max_geodes(b, 32))
        .product()
}

impl FromStr for Blueprint {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words = s.split_whitespace().collect_vec();
        Ok(Blueprint {
            index: words[1].strip_suffix(":").unwrap().parse().unwrap(),
            ore_robot_cost: Resources { ore: words[6].parse().unwrap(), ..Default::default() },
            clay_robot_cost: Resources { ore: words[12].parse().unwrap(), ..Default::default() },
            obsidian_robot_cost: Resources { ore: words[18].parse().unwrap(), clay: words[21].parse().unwrap(), ..Default::default() },
            geode_robot_cost: Resources { ore: words[27].parse().unwrap(), obsidian: words[30].parse().unwrap(), ..Default::default() },
        })

        //Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 17 clay. Each geode robot costs 4 ore and 16 obsidian.
    }
}

fn max_geodes(blueprint: &Blueprint, minutes: usize) -> usize {
    let production_upper_bound = minutes;
    let inital = SearchState {
        resources: Default::default(),
        robots: Resources { ore: 1, ..Default::default() },
        remaining_time: minutes,
    };
    let result: (Vec<SearchState>, usize) = astar(
        &inital,
        |n| successors(n, blueprint, production_upper_bound),
        |n| {
            let current_robot_remaining_production = n.robots.geode * n.remaining_time;
            let future_robot_potential_production: usize = (1..n.remaining_time).sum();
            let adjustment = n.remaining_time * production_upper_bound;
            adjustment - current_robot_remaining_production - future_robot_potential_production
        },
        |n| n.remaining_time == 0,
    ).unwrap();

    (result.1 as i32 - (minutes * production_upper_bound) as i32).abs() as usize
}

fn successors(search_state: &SearchState, blueprint: &Blueprint, production_upper_bound: usize) -> Vec<(SearchState, usize)> {
    let mut succs = vec![];
    let cost = production_upper_bound - search_state.robots.geode;

    let time_to_robot = 1 + time_to(&search_state.resources, &search_state.robots, &blueprint.geode_robot_cost);
    if time_to_robot < search_state.remaining_time {
        succs.push((SearchState {
            resources: search_state.resources.plus(&search_state.robots.times(time_to_robot)).minus(&blueprint.geode_robot_cost),
            robots: Resources { geode: search_state.robots.geode + 1, ..search_state.robots },
            remaining_time: search_state.remaining_time - time_to_robot,
        }, cost * time_to_robot));
    }

    let time_to_robot = 1 + time_to(&search_state.resources, &search_state.robots, &blueprint.obsidian_robot_cost);
    if time_to_robot < search_state.remaining_time {
        succs.push((SearchState {
            resources: search_state.resources.plus(&search_state.robots.times(time_to_robot)).minus(&blueprint.obsidian_robot_cost),
            robots: Resources { obsidian: search_state.robots.obsidian + 1, ..search_state.robots },
            remaining_time: search_state.remaining_time - time_to_robot,
        }, cost * time_to_robot));
    }

    let time_to_robot = 1 + time_to(&search_state.resources, &search_state.robots, &blueprint.clay_robot_cost);
    if time_to_robot < search_state.remaining_time {
        succs.push((SearchState {
            resources: search_state.resources.plus(&search_state.robots.times(time_to_robot)).minus(&blueprint.clay_robot_cost),
            robots: Resources { clay: search_state.robots.clay + 1, ..search_state.robots },
            remaining_time: search_state.remaining_time - time_to_robot,
        }, cost * time_to_robot));
    }

    let time_to_robot = 1 + time_to(&search_state.resources, &search_state.robots, &blueprint.ore_robot_cost);
    if time_to_robot < search_state.remaining_time {
        succs.push((SearchState {
            resources: search_state.resources.plus(&search_state.robots.times(time_to_robot)).minus(&blueprint.ore_robot_cost),
            robots: Resources { ore: search_state.robots.ore + 1, ..search_state.robots },
            remaining_time: search_state.remaining_time - time_to_robot,
        }, cost * time_to_robot));
    }

    if succs.is_empty() {
        succs.push((SearchState {
            resources: search_state.resources.plus(&search_state.robots.times(search_state.remaining_time)),
            robots: search_state.robots.clone(),
            remaining_time: 0,
        }, cost * search_state.remaining_time));
    }
    succs
}

fn time_to(current: &Resources, income: &Resources, target: &Resources) -> usize {
    vec![
        calc_time_to(current.ore, income.ore, target.ore),
        calc_time_to(current.clay, income.clay, target.clay),
        calc_time_to(current.obsidian, income.obsidian, target.obsidian),
        calc_time_to(current.geode, income.geode, target.geode),
    ].into_iter().max().unwrap()
}

const INFINITY: usize = 1_000_000_000;

fn calc_time_to(current: usize, income: usize, target: usize) -> usize {
    if target <= current { return 0; }
    if income == 0 { return INFINITY; }
    let missing = target - current;
    if missing % income == 0 {
        missing / income
    } else {
        1 + missing / income
    }
}

#[derive(Default, Eq, PartialEq, Hash, Clone, Debug)]
struct Resources {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

impl Resources {
    fn minus(&self, other: &Resources) -> Resources {
        Resources {
            ore: self.ore - other.ore,
            clay: self.clay - other.clay,
            obsidian: self.obsidian - other.obsidian,
            geode: self.geode - other.geode,
        }
    }

    fn plus(&self, other: &Resources) -> Resources {
        Resources {
            ore: self.ore + other.ore,
            clay: self.clay + other.clay,
            obsidian: self.obsidian + other.obsidian,
            geode: self.geode + other.geode,
        }
    }

    fn times(&self, factor: usize) -> Resources {
        Resources {
            ore: self.ore * factor,
            clay: self.clay * factor,
            obsidian: self.obsidian * factor,
            geode: self.geode * factor,
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Blueprint {
    index: usize,
    ore_robot_cost: Resources,
    clay_robot_cost: Resources,
    obsidian_robot_cost: Resources,
    geode_robot_cost: Resources,
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct SearchState {
    resources: Resources,
    robots: Resources,
    remaining_time: usize,
}


#[cfg(test)]
mod tests {
    use crate::day19::*;


    #[test]
    fn should_parse_blueprint() {
        let input = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 17 clay. Each geode robot costs 4 ore and 16 obsidian.".to_string();
        assert_eq!(input.parse(), Ok(Blueprint {
            index: 1,
            ore_robot_cost: Resources { ore: 4, ..Default::default() },
            clay_robot_cost: Resources { ore: 4, ..Default::default() },
            obsidian_robot_cost: Resources { ore: 4, clay: 17, ..Default::default() },
            geode_robot_cost: Resources { ore: 4, obsidian: 16, ..Default::default() },
        }))
    }

    #[test]
    fn should_solve_part_1_on_example() {
        let blueprint = Blueprint {
            index: 1,
            ore_robot_cost: Resources { ore: 4, ..Default::default() },
            clay_robot_cost: Resources { ore: 2, ..Default::default() },
            obsidian_robot_cost: Resources { ore: 3, clay: 14, ..Default::default() },
            geode_robot_cost: Resources { ore: 2, obsidian: 7, ..Default::default() },
        };

        assert_eq!(max_geodes(&blueprint, 24), 9)
    }

    #[test]
    fn should_solve_part_2_on_example() {
        let blueprint = Blueprint {
            index: 1,
            ore_robot_cost: Resources { ore: 4, ..Default::default() },
            clay_robot_cost: Resources { ore: 2, ..Default::default() },
            obsidian_robot_cost: Resources { ore: 3, clay: 14, ..Default::default() },
            geode_robot_cost: Resources { ore: 2, obsidian: 7, ..Default::default() },
        };

        assert_eq!(max_geodes(&blueprint, 32), 56)
    }
}
