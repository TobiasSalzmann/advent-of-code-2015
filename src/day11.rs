use itertools::Itertools;
use crate::util;

pub fn main() {
    let input = util::parse_strings("resources/day11.txt");
    let monkeys = input
        .split(|s| s.is_empty())
        .map(|rm| parse_monkey(rm.to_vec()))
        .collect_vec();

    println!(
        "Day 11, Part 1: {:?} monkey business",
        calculate_monkey_business(&mut monkeys.clone())
    );

    println!(
        "Day 11, Part 2: {:?} monkey business",
        calculate_monkey_business_2(&mut monkeys.clone())
    );
}

fn parse_monkey(raw_monkey: Vec<String>) -> Monkey {
    Monkey {
        items: parse_items(&raw_monkey),
        operation: parse_operation(&raw_monkey),
        action: parse_action(&raw_monkey),
        items_inspected: 0,
    }
}

fn parse_items(raw_monkey: &Vec<String>) -> Vec<i128> {
    let items: Vec<i128> = raw_monkey[1]
        .strip_prefix("  Starting items: ").unwrap()
        .split(", ")
        .map(|s| s.parse().unwrap())
        .collect_vec();
    items
}

fn parse_operation(raw_monkey: &Vec<String>) -> Inspection {
    let (raw_operator, raw_operand): (&str, &str) = raw_monkey[2]
        .strip_prefix("  Operation: new = old ").unwrap()
        .split(" ")
        .collect_tuple().unwrap();
    let operation = Inspection {
        operator: match raw_operator {
            "+" => Operator::Plus,
            "*" => Operator::Times,
            _ => panic!("Expected + or * as operator in operation")
        },
        operand: match raw_operand {
            "old" => Operand::Old,
            value => Operand::Const(value.parse().unwrap())
        },
    };
    operation
}

fn parse_action(raw_monkey: &Vec<String>) -> Action {
    let modulus: i128 = raw_monkey[3]
        .strip_prefix("  Test: divisible by ").unwrap()
        .parse().unwrap();
    let true_target: i128 = raw_monkey[4]
        .strip_prefix("    If true: throw to monkey ").unwrap()
        .parse().unwrap();
    let false_target: i128 = raw_monkey[5]
        .strip_prefix("    If false: throw to monkey ").unwrap()
        .parse().unwrap();
    let action = Action { modulus, true_target, false_target };
    action
}

fn calculate_monkey_business(monkeys: &mut Vec<Monkey>) -> u128 {
    for _ in 0..20 {
        advance_round(monkeys, |w| w / 3)
    }
    extract_monkey_business(monkeys)
}

fn calculate_monkey_business_2(monkeys: &mut Vec<Monkey>) -> u128 {
    let modulus: i128 = monkeys.iter().map(|m| m.action.modulus).product();
    for _ in 0..10000 {
        advance_round(monkeys, |w| w % modulus)
    }
    extract_monkey_business(monkeys)
}

fn extract_monkey_business(monkeys: &Vec<Monkey>) -> u128 {
    monkeys.iter()
        .map(|m| m.items_inspected as u128)
        .sorted()
        .rev()
        .take(2)
        .product()
}

fn advance_round(monkeys: &mut Vec<Monkey>, modify_worry: impl Fn(i128) -> i128) {
    for i in 0..monkeys.len() {
        for item in monkeys[i].items.clone() {
            let worry_level = modify_worry(monkeys[i].operation.apply(item));
            let target_monkey = monkeys[i].action.apply(worry_level);
            monkeys[target_monkey as usize].items.push(worry_level);
        }
        monkeys[i].items_inspected += monkeys[i].items.len() as u32;
        monkeys[i].items = vec![];
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Operator {
    Plus,
    Times,
}

#[derive(Clone, Debug, PartialEq)]
enum Operand {
    Old,
    Const(i128),
}

#[derive(Clone, Debug, PartialEq)]
struct Inspection {
    operator: Operator,
    operand: Operand,
}

impl Inspection {
    pub fn apply(&self, worry_level: i128) -> i128 {
        let evaluated_operand = match self.operand {
            Operand::Old => worry_level,
            Operand::Const(num) => num,
        };
        match self.operator {
            Operator::Plus => worry_level + evaluated_operand,
            Operator::Times => worry_level * evaluated_operand,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Action {
    modulus: i128,
    true_target: i128,
    false_target: i128,
}

impl Action {
    pub fn apply(&self, worry_level: i128) -> i128 {
        if worry_level % self.modulus == 0 {
            self.true_target
        } else {
            self.false_target
        }
    }
}


#[derive(Clone, Debug, PartialEq)]
struct Monkey {
    items: Vec<i128>,
    operation: Inspection,
    action: Action,
    items_inspected: u32,
}

#[cfg(test)]
mod tests {
    use crate::day11::*;
    use crate::day11::Operand::{Const, Old};
    use crate::day11::Operator::{Plus, Times};

    #[test]
    fn parses_a_monkey_from_input() {
        let input = vec![
            "Monkey 0:".to_string(),
            "  Starting items: 79, 98".to_string(),
            "  Operation: new = old * 19".to_string(),
            "  Test: divisible by 23".to_string(),
            "    If true: throw to monkey 2".to_string(),
            "    If false: throw to monkey 3".to_string(),
        ];
        let monkey = Monkey {
            items: vec![79, 98],
            operation: Inspection { operator: Times, operand: Const(19) },
            action: Action { modulus: 23, true_target: 2, false_target: 3 },
            items_inspected: 0,
        };
        assert_eq!(parse_monkey(input), monkey);
    }

    #[test]
    fn operation_inspects_item() {
        assert_eq!(Inspection { operator: Plus, operand: Old }.apply(10), 20);
        assert_eq!(Inspection { operator: Times, operand: Old }.apply(10), 100);
        assert_eq!(Inspection { operator: Plus, operand: Const(5) }.apply(10), 15);
        assert_eq!(Inspection { operator: Times, operand: Const(5) }.apply(10), 50);
    }

    #[test]
    fn action_throws_item() {
        assert_eq!(Action { modulus: 23, true_target: 2, false_target: 3 }.apply(46), 2);
        assert_eq!(Action { modulus: 22, true_target: 2, false_target: 3 }.apply(46), 3);
        assert_eq!(Action { modulus: 3, true_target: 5, false_target: 6 }.apply(6), 5);
        assert_eq!(Action { modulus: 3, true_target: 5, false_target: 6 }.apply(2), 6);
    }

    #[test]
    fn monkey_takes_turn() {
        let mut start_monkeys = vec![
            Monkey {
                items: vec![79, 98],
                operation: Inspection { operator: Times, operand: Const(19) },
                action: Action { modulus: 23, true_target: 2, false_target: 3 },
                items_inspected: 0,
            },
            Monkey {
                items: vec![54, 65, 75, 74],
                operation: Inspection { operator: Plus, operand: Const(6) },
                action: Action { modulus: 19, true_target: 2, false_target: 0 },
                items_inspected: 0,
            },
            Monkey {
                items: vec![79, 60, 97],
                operation: Inspection { operator: Times, operand: Old },
                action: Action { modulus: 13, true_target: 1, false_target: 3 },
                items_inspected: 0,
            },
            Monkey {
                items: vec![74],
                operation: Inspection { operator: Plus, operand: Const(3) },
                action: Action { modulus: 17, true_target: 0, false_target: 1 },
                items_inspected: 0,
            },
        ];

        advance_round(&mut start_monkeys, |w| w / 3);

        assert_eq!(start_monkeys[0].items, vec![20, 23, 27, 26]);
        assert_eq!(start_monkeys[1].items, vec![2080, 25, 167, 207, 401, 1046]);
        assert_eq!(start_monkeys[2].items, vec![]);
        assert_eq!(start_monkeys[3].items, vec![]);
    }

    #[test]
    fn calculates_monkey_business_for_parsed_example() {
        let mut start_monkeys = vec![
            Monkey {
                items: vec![79, 98],
                operation: Inspection { operator: Times, operand: Const(19) },
                action: Action { modulus: 23, true_target: 2, false_target: 3 },
                items_inspected: 0,
            },
            Monkey {
                items: vec![54, 65, 75, 74],
                operation: Inspection { operator: Plus, operand: Const(6) },
                action: Action { modulus: 19, true_target: 2, false_target: 0 },
                items_inspected: 0,
            },
            Monkey {
                items: vec![79, 60, 97],
                operation: Inspection { operator: Times, operand: Old },
                action: Action { modulus: 13, true_target: 1, false_target: 3 },
                items_inspected: 0,
            },
            Monkey {
                items: vec![74],
                operation: Inspection { operator: Plus, operand: Const(3) },
                action: Action { modulus: 17, true_target: 0, false_target: 1 },
                items_inspected: 0,
            },
        ];

        assert_eq!(calculate_monkey_business(&mut start_monkeys), 10605);
    }

    #[test]
    fn calculates_monkey_business_for_parsed_example_for_part_2() {
        let mut start_monkeys = vec![
            Monkey {
                items: vec![79, 98],
                operation: Inspection { operator: Times, operand: Const(19) },
                action: Action { modulus: 23, true_target: 2, false_target: 3 },
                items_inspected: 0,
            },
            Monkey {
                items: vec![54, 65, 75, 74],
                operation: Inspection { operator: Plus, operand: Const(6) },
                action: Action { modulus: 19, true_target: 2, false_target: 0 },
                items_inspected: 0,
            },
            Monkey {
                items: vec![79, 60, 97],
                operation: Inspection { operator: Times, operand: Old },
                action: Action { modulus: 13, true_target: 1, false_target: 3 },
                items_inspected: 0,
            },
            Monkey {
                items: vec![74],
                operation: Inspection { operator: Plus, operand: Const(3) },
                action: Action { modulus: 17, true_target: 0, false_target: 1 },
                items_inspected: 0,
            },
        ];

        assert_eq!(calculate_monkey_business_2(&mut start_monkeys), 2713310158);
    }
}







