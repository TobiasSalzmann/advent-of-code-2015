use std::cmp::Ordering;
use std::str::FromStr;
use itertools::Itertools;
use serde_json::{Error, Value};
use serde_json::Value::{Array, Number};
use crate::day13::Packet::{List, Num};
use crate::util;

pub fn main() {
    let input = util::parse_strings("resources/day13.txt");
    let packets: Vec<(Packet, Packet)> = input
        .split(|s| s.is_empty())
        .map(|packets| (packets[0].parse().unwrap(), packets[1].parse().unwrap()))
        .collect_vec();

    println!(
        "Day 13, Part 1: Ordered packet sum: {:?}", sum_ordered_packets(packets.clone())
    );

    println!(
        "Day 13, Part 2: Decoder key: {:?}", decoder_key(packets)
    );
}

fn sum_ordered_packets(packet_pairs: Vec<(Packet, Packet)>) -> usize {
    packet_pairs.into_iter()
        .enumerate()
        .map(|(i, (p1, p2))| {
            if p1 < p2 { i + 1 } else { 0 }
        }).sum()
}

fn decoder_key(packet_pairs: Vec<(Packet, Packet)>) -> usize {
    let marker1 = List(vec![List(vec![Num(2)])]);
    let marker2 = List(vec![List(vec![Num(6)])]);
    packet_pairs.into_iter()
        .flat_map(|(a, b)| [a, b])
        .chain([marker1.clone(), marker2.clone()])
        .sorted()
        .positions(|p| p.clone() == marker1 || p.clone() == marker2)
        .map(|x| x + 1)
        .product()
}

#[derive(Eq, PartialEq, Clone)]
enum Packet {
    List(Vec<Packet>),
    Num(i32),
}

fn packet_from_value(v: Value) -> Packet {
    match v {
        Number(x) => Num(x.as_i64().unwrap() as i32),
        Array(vs) => List(vs.into_iter().map(packet_from_value).collect_vec()),
        _ => panic!("aaaahhhhhh")
    }
}

impl FromStr for Packet {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str::<Value>(s).map(|v| packet_from_value(v))
    }
}

impl PartialOrd<Self> for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Num(x), Num(y)) => x.cmp(y),
            (xs, Num(y)) => xs.cmp(&List(vec![Num(y.clone())])),
            (Num(x), ys) => List(vec![Num(x.clone())]).cmp(ys),
            (List(xs), List(ys)) => xs.cmp(ys)
        }
    }
}

#[cfg(test)]
mod tests {
    

    #[test]
    fn should_do_stuff() {}
}







