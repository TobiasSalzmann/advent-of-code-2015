use std::str::FromStr;
use itertools::{fold, Itertools, repeat_n};
use petgraph::visit::Walker;
use crate::util;

pub fn main() {
    let input: Vec<Action> = util::parse_from_strings("resources/day9.txt");

    println!(
        "Day 11, Part 1: {:?}", part1(input.clone())
    );

    // println!(
    //     "Day 11, Part 2:\n{}", part2(input)
    // );
}

fn part1(actions: Vec<Action>) -> usize {
    let mut states = vec![State { head_pos: (0, 0), tail_pos: (0, 0) }];
    let flat_actions = actions.iter()
        .flat_map(|Action { dir, times }| (1..=times.clone()).map(|_| Action { dir: dir.clone(), times: 1 }));

    for a in flat_actions {
        let new_state = states.last().unwrap().move_head(a).move_tail();
        states.push(new_state)
    }

    states.iter()
        .map(|s| s.tail_pos)
        .unique()
        .count()
}


// fn part2(commands: Vec<Command>) -> String {
//     let v = compute_signal(commands);
//
//     v.iter()
//         .take(240)
//         .enumerate()
//         .map(|(i, n)| {
//             let j = (i as i32) % 40;
//             if j >= n-1 && j <= n+1 {'#'} else {'.'}
//         })
//         .chunks(40).into_iter()
//         .map(|cs| cs.into_iter().join(""))
//         .join("\n")
// }

#[derive(Debug)]
struct State {
    head_pos: (i32, i32),
    tail_pos: (i32, i32),
}

impl State {
    fn move_head(&self, action: Action) -> State {
        let (mut head_x, mut head_y) = self.head_pos;
        match action.dir {
            'U' => head_y -= 1,
            'D' => head_y += 1,
            'L' => head_x -= 1,
            'R' => head_x += 1,
            _ => {}
        }
        State {
            head_pos: (head_x, head_y),
            ..*self
        }
    }

    fn move_tail(&self) -> State {
        let (head_x, head_y) = self.head_pos;
        let (mut tail_x, mut tail_y) = self.tail_pos;
        let delta_x = head_x - tail_x;
        let delta_y = head_y - tail_y;

        if delta_x != 0 && delta_y != 0 && (delta_x.abs() > 1 || delta_y.abs() > 1) {
            tail_x += delta_x.signum();
            tail_y += delta_y.signum();
        } else if delta_y == 0 && delta_x.abs() > 1 {
            tail_x += delta_x.signum();
        } else if delta_x == 0 && delta_y.abs() > 1 {
            tail_y += delta_y.signum();
        }

        State {
            tail_pos: (tail_x, tail_y),
            ..*self
        }
    }
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







