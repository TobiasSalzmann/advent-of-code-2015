use std::collections::HashMap;
use std::str::FromStr;
use itertools::{Itertools};
use crate::day7::Instruction::{And, LShift, Not, Or, RShift, Copy};
use crate::day7::ValueSource::{Const, Register};

use crate::util;

pub fn main() {
    let input = util::parse_from_strings("resources/day7.txt");

    println!("Day 7, Part 1: {}", execute(&input));
    println!("Day 7, Part 2: {}", execute_2(&input));
}

// fn execute(instructions: &Vec<Instruction>) -> u16 {
//     let lookup: HashMap<String, Instruction> = instructions.iter()
//         .map(|i| match i {
//             Copy { to, .. } => (to.clone(), i.clone()),
//             Not { to, .. } => (to.clone(), i.clone()),
//             Or { to, .. } => (to.clone(), i.clone()),
//             And { to, .. } => (to.clone(), i.clone()),
//             LShift { to, .. } => (to.clone(), i.clone()),
//             RShift { to, .. } => (to.clone(), i.clone()),
//         }).collect();
//
//     lookup.get("a").unwrap().eval(&lookup)
// }

fn execute(instructions: &Vec<Instruction>) -> u16 {
    let mut mem: HashMap<String, u16> = HashMap::new();
    run_instructions(instructions, &mut mem);
    mem.get("a").unwrap().clone()
}

fn execute_2(instructions: &Vec<Instruction>) -> u16 {
    let mut mem: HashMap<String, u16> = HashMap::new();
    run_instructions(instructions, &mut mem);
    let v1 = mem.get("a").unwrap().clone();
    let mut mem2: HashMap<String, u16> = HashMap::new();
    let mut instructions2 = instructions.clone();
    for i in instructions2.iter_mut() {
        match i {
            Copy { from, to } if to == "b" => {
                *i = Copy { from: ValueSource::Const(v1), to: to.clone() }
            }
            _ => {}
        }
    }
    run_instructions(&instructions2, &mut mem2);
    mem2.get("a").unwrap().clone()
}

fn run_instructions(instructions: &Vec<Instruction>, mem: &mut HashMap<String, u16>) {
    let ev = |vs: ValueSource, mem: &HashMap<String, u16>| match vs.clone() {
        ValueSource::Const(x) => x,
        ValueSource::Register(r) => mem.get(&r).unwrap_or(&0u16).clone()
    };
    let mut tainted = true;
    while tainted {
        tainted = false;
        for instruction in instructions {
            let (k, v) = match instruction {
                Copy { from, to } => {
                    (to.clone(), ev(from.clone(), mem))
                }
                Not { from, to } => {
                    (to.clone(), !ev(from.clone(), mem))
                }
                Or { from_1, from_2, to } => {
                    (to.clone(), ev(from_1.clone(), mem) | ev(from_2.clone(), mem))
                }
                And { from_1, from_2, to } => {
                    (to.clone(), ev(from_1.clone(), mem) & ev(from_2.clone(), mem))
                }
                LShift { from, value, to } => {
                    (to.clone(), ev(from.clone(), mem) << ev(value.clone(), mem))
                }
                RShift { from, value, to } => {
                    (to.clone(), ev(from.clone(), mem) >> ev(value.clone(), mem))
                }
            };
            let update = mem.insert(k, v);
            match update {
                Some(old_v) if old_v == v => {}
                _ => tainted = true
            }
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Hash, Clone)]
enum ValueSource {
    Const(u16),
    Register(String),
}

impl ValueSource {
    fn from(s: &str) -> Self {
        match s.parse() {
            Ok(x) => Const(x),
            Err(_) => Register(s.to_string()),
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Hash, Clone)]
enum Instruction {
    Copy { from: ValueSource, to: String },
    Not { from: ValueSource, to: String },
    Or { from_1: ValueSource, from_2: ValueSource, to: String },
    And { from_1: ValueSource, from_2: ValueSource, to: String },
    LShift { from: ValueSource, value: ValueSource, to: String },
    RShift { from: ValueSource, value: ValueSource, to: String },
}

// impl Instruction {
//     fn eval(&self, lookup: &HashMap<String, Instruction>) -> u16 {
//         println!("{:?}", self);
//         let get = |s: String| lookup.get(&s).unwrap();
//         let ev = |vs: ValueSource| match vs.clone() {
//             ValueSource::Const(x) => x,
//             ValueSource::Register(r) => get(r).eval(lookup)
//         };
//         match self {
//             Copy { from, .. } => ev(from.clone()),
//             Not { from, .. } => !ev(from.clone()),
//             Or { from_1, from_2, .. } => ev(from_1.clone()) | ev(from_2.clone()),
//             And { from_1, from_2, .. } => ev(from_1.clone()) & ev(from_2.clone()),
//             LShift { from, value, .. } => ev(from.clone()) << value,
//             RShift { from, value, .. } => ev(from.clone()) >> value,
//         }
//     }
// }

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match &s.split_whitespace().collect_vec()[..] {
            [value, "->", to] =>
                Copy { to: to.to_string(), from: ValueSource::from(value) },
            ["NOT", from, "->", to] =>
                Not { from: ValueSource::from(from), to: to.to_string() },
            [from_1, "OR", from_2, "->", to] =>
                Or { from_1: ValueSource::from(from_1), from_2: ValueSource::from(from_2), to: to.to_string() },
            [from_1, "AND", from_2, "->", to] =>
                And { from_1: ValueSource::from(from_1), from_2: ValueSource::from(from_2), to: to.to_string() },
            [from, "LSHIFT", value, "->", to] =>
                LShift { from: ValueSource::from(from), to: to.to_string(), value: ValueSource::from(value) },
            [from, "RSHIFT", value, "->", to] =>
                RShift { from: ValueSource::from(from), to: to.to_string(), value: ValueSource::from(value) },
            unknown => panic!("{:?}", unknown)
        })
    }
}