use serde_json::{Value};
use itertools::{Itertools};


use crate::util;

pub fn main() {
    let input = util::parse_string("resources/day12.txt");
    let json: Value = serde_json::from_str(&input).unwrap();


    println!("Day 11 Part 1: {}", add_numbers(&json, false));
    println!("Day 11 Part 2: {}", add_numbers(&json, true));
}

fn add_numbers(json: &Value, ignore_red: bool) -> i64 {
    match json {
        Value::Number(n) => n.as_i64().unwrap() ,
        Value::String(_) => 0,
        Value::Array(values) => values.iter().map(|v| add_numbers(v, ignore_red)).sum(),
        Value::Object(mappings) if ignore_red
            && mappings.values().contains(&Value::String("red".to_string())) => 0,
        Value::Object(mappings) => mappings.values().map(|v| add_numbers(v, ignore_red)).sum(),
        _ => panic!(),
    }
}