use std::ops::Deref;
use std::str::FromStr;
use itertools::{fold, Itertools, repeat_n};
use petgraph::visit::Walker;
use crate::util;

pub fn main() {
    let input: Vec<Action> = util::parse_from_strings("resources/day9.txt");

    println!(
        "Day 9, Part 1: {:?}", part(input.clone(), 2)
    );

    println!(
        "Day 9, Part 2: {:?}", part(input.clone(), 10)
    );
}

fn part(actions: Vec<Action>, number_of_knots: usize) -> usize {
    let mut states = vec![State { knots: (1..=number_of_knots).map(|_| (0,0)).collect_vec() }];
    let flat_actions = actions.iter()
        .flat_map(|Action { dir, times }| (1..=times.clone()).map(|_| Action { dir: dir.clone(), times: 1 }));

    for a in flat_actions {
        let new_state = states.last().unwrap().advance(a);
        states.push(new_state)
    }

    states.iter()
        .map(|s| s.tail_pos())
        .unique()
        .count()
}

#[derive(Debug, Clone)]
struct State {
    knots: Vec<(i32, i32)>
}

impl State {
    fn advance(&self, action: Action) -> State {
        let mut it = self.knots.iter();
        let new_head = move_head(it.next().unwrap(), action);
        let mut new_knots = vec![new_head];
        for knot in it {
            let new_knot = follow(new_knots.last().unwrap(), knot);
            new_knots.push(new_knot);
        }
        State {
            knots: new_knots
        }
    }

    fn tail_pos(&self) -> (i32, i32) {
        self.knots.last().unwrap().clone()
    }
}

fn move_head(knot: &(i32, i32), action: Action) -> (i32, i32) {
    let (mut head_x, mut head_y) = knot;
    match action.dir {
        'U' => head_y -= 1,
        'D' => head_y += 1,
        'L' => head_x -= 1,
        'R' => head_x += 1,
        _ => {}
    }
    (head_x, head_y)
}

fn follow(leader: &(i32, i32), follower: &(i32, i32)) -> (i32, i32) {
    let (head_x, head_y) = leader;
    let (mut follow_x, mut follow_y) = follower;
    let delta_x = head_x - follow_x;
    let delta_y = head_y - follow_y;

    if delta_x != 0 && delta_y != 0 && (delta_x.abs() > 1 || delta_y.abs() > 1) {
        follow_x += delta_x.signum();
        follow_y += delta_y.signum();
    } else if delta_y == 0 && delta_x.abs() > 1 {
        follow_x += delta_x.signum();
    } else if delta_x == 0 && delta_y.abs() > 1 {
        follow_y += delta_y.signum();
    }

    (follow_x, follow_y)
}

#[derive(Clone)]
struct Action {
    dir: char,
    times: i32,
}

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s.split(" ").collect_vec()[..] {
            [dir, num] => Ok(Action {
                dir: dir.chars().next().unwrap(),
                times: num.parse().unwrap(),
            }),
            _ => panic!("aaaaa")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day9::*;

    #[test]
    fn should_do_stuff() {}
}







