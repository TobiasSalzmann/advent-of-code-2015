
use std::collections::HashMap;

use std::str::FromStr;
use itertools::{Itertools};
use crate::day21::Expression::{Const, Div, Minus, Mul, Plus};
use crate::util;

pub fn main() {
    let input: Vec<Monkey> = util::parse_from_strings("resources/day21.txt");

    println!("Day 21, Part 1: {:?}", root_monkey_number(input.clone()));
    println!("Day 21, Part 2: {:?}", find_human_number(input.clone()));
}

fn root_monkey_number(monkeys: Vec<Monkey>) -> i128 {
    let map: HashMap<String, &Monkey> = monkeys.iter()
        .map(|m| (m.name.clone(), m)).collect();
    let root = *map.get("root").unwrap();
    root.eval_with(&map)
}

fn find_human_number(monkeys: Vec<Monkey>) -> i128 {
    let map: HashMap<String, &Monkey> = monkeys.iter()
        .map(|m| (m.name.clone(), m)).collect();
    let root = *map.get("root").unwrap();
    root.find_human_number_with(0, &map)
}

#[derive(Clone)]
struct Monkey {
    name: String,
    expression: Expression,
}

impl Monkey {
    fn eval_with(&self, monkeys: &HashMap<String, &Monkey>) -> i128 {
        let eval = |a: String| {
            monkeys.get(&a).unwrap().eval_with(monkeys)
        };

        match self.expression.clone() {
            Const(n) => n,
            Plus(a, b) => eval(a) + eval(b),
            Minus(a, b) => eval(a) - eval(b),
            Mul(a, b) => eval(a) * eval(b),
            Div(a, b) => eval(a) / eval(b),
        }
    }

    fn find_human_number_with(&self, expectation: i128, monkeys: &HashMap<String, &Monkey>) -> i128 {
        let (left, right) = match self.children_with(monkeys) {
            None => return expectation,
            Some(pair) => pair
        };

        let left_has_human = left.depends_on_human(monkeys);
        let (monkey_with_human, monkey_without_human) = if left_has_human {
            (left, right)
        } else { (right, left) };

        let value = monkey_without_human.eval_with(monkeys);

        let new_expectation = match self.expression.clone() {
            _ if self.name == "root" => value,
            Plus(_, _) => expectation - value,
            Minus(_, _) => if left_has_human {expectation + value} else { value - expectation },
            Mul(_, _) => expectation / value,
            Div(_, _) => if left_has_human {expectation * value} else {value / expectation},
            _ => panic!(),
        };
        monkey_with_human.find_human_number_with(new_expectation, monkeys)
    }

    fn depends_on_human(&self, monkeys: &HashMap<String, &Monkey>) -> bool {
        if self.name == "humn" {
            return true;
        }
        let depends_on = |a: String| {
            monkeys.get(&a).unwrap().depends_on_human(monkeys)
        };

        match self.expression.clone() {
            Const(_) => false,
            Plus(a, b) => depends_on(a) || depends_on(b),
            Minus(a, b) => depends_on(a) || depends_on(b),
            Mul(a, b) => depends_on(a) || depends_on(b),
            Div(a, b) => depends_on(a) || depends_on(b),
        }
    }

    fn children_with<'a>(&self, monkeys: &'a HashMap<String, &Monkey>) -> Option<(&'a Monkey, &'a Monkey)> {
        match self.expression.clone() {
            Const(_) => None,
            Plus(a, b) => Some((monkeys.get(&a).unwrap(), monkeys.get(&b).unwrap())),
            Minus(a, b) => Some((monkeys.get(&a).unwrap(), monkeys.get(&b).unwrap())),
            Mul(a, b) => Some((monkeys.get(&a).unwrap(), monkeys.get(&b).unwrap())),
            Div(a, b) => Some((monkeys.get(&a).unwrap(), monkeys.get(&b).unwrap())),
        }
    }
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, exp) = s.split_once(": ").unwrap();
        let expression = match exp.split(" ").collect_vec()[..] {
            [a, "+", b] => Plus(a.to_string(), b.to_string()),
            [a, "-", b] => Minus(a.to_string(), b.to_string()),
            [a, "*", b] => Mul(a.to_string(), b.to_string()),
            [a, "/", b] => Div(a.to_string(), b.to_string()),
            [n] => Const(n.parse().unwrap()),
            _ => panic!()
        };
        Ok(Monkey { name: name.to_string(), expression })
    }
}

#[derive(Clone)]
enum Expression {
    Const(i128),
    Plus(String, String),
    Minus(String, String),
    Mul(String, String),
    Div(String, String),
}


#[cfg(test)]
mod tests {
    #[test]
    fn should_mix_on_example() {}
}
