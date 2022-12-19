use std::collections::HashMap;
use std::str::FromStr;
use itertools::Itertools;
use pathfinding::directed::dijkstra;
use pathfinding::prelude::dijkstra;
use crate::util;

pub fn main() {
    let input: Vec<Node<String>> = util::parse_from_strings("resources/day16.txt");

    println!("Day 16, Part 1: {:?}", max_releasable_pressure(input.clone()));
}

fn max_releasable_pressure(nodes: Vec<Node<String>>) -> i64 {
    let node_lookup = convert_nodes(&nodes);
    let maximum_flow = node_lookup.values().map(|n| n.flow_rate).sum();
    let start = SearchState {
        node: 1,
        open_valves: 0,
        current_flow: 0,
        elapsed_time: 0,
    };
    let result: (Vec<SearchState>, usize) = dijkstra(
        &start,
        |n| successors(n, &node_lookup, maximum_flow),
        |n| n.elapsed_time == 30,
    ).unwrap();

    (result.1 as i64 - (30 * maximum_flow as i64)).abs()
}

fn successors(search_state: &SearchState, node_lookup: &HashMap<u64, Node<u64>>, maximum_flow: usize) -> Vec<(SearchState, usize)> {
    let mut succs = vec![];
    let node = node_lookup.get(&search_state.node).unwrap();

    let valve_already_open = (search_state.open_valves | search_state.node) == search_state.open_valves;
    if node.flow_rate > 0 && !valve_already_open {
        let state = SearchState {
            node: search_state.node,
            open_valves: search_state.open_valves | search_state.node,
            current_flow: search_state.current_flow + node.flow_rate,
            elapsed_time: search_state.elapsed_time + 1,
        };
        let cost = maximum_flow - search_state.current_flow;
        succs.push((state, cost));
    }

    for (_ , neighbour) in &node.neighbours {
        let state = SearchState {
            node: neighbour.clone(),
            open_valves: search_state.open_valves,
            current_flow: search_state.current_flow,
            elapsed_time: search_state.elapsed_time + 1,
        };
        let cost = maximum_flow - search_state.current_flow;
        succs.push((state, cost));
    }

    succs
}


// fn max_releasable_pressure(nodes: Vec<Node<String>>) -> i32 {
//     let nodes_lookup = convert_nodes(&nodes);
//
//     let edges = nodes_lookup.values()
//         .flat_map(|n| n.neighbours.iter().map(|(_, nb)| (n.name, nb.clone())));
//     let graph: DiGraphMap<u64, i32> = GraphMap::from_edges(edges);
//     let result = floyd_warshall(&graph, |_| 1).unwrap();
//     let important_nodes = nodes_lookup.values()
//         .filter(|n| n.flow_rate > 0 || n.name == 1)
//         .map(|n| {
//             let neighbours = result.iter()
//                 .filter(|((a, b), d)| a.clone() == n.name && b.clone() != n.name)
//                 .filter(|((a, b), d)| nodes_lookup.get(b).unwrap().flow_rate > 0 || nodes_lookup.get(b).unwrap().name == 1)
//                 .map(|((a, b), d)| (d.clone(), b.clone()))
//                 .collect_vec();
//             Node {
//                 neighbours,
//                 ..n.clone()
//             }
//         })
//         .collect_vec();
//     let important_nodes_lookup: HashMap<u64, Node<u64>> = important_nodes.iter()
//         .map(|n| (n.name, n.clone()))
//         .collect();
//
//     let total_valves = nodes.iter()
//         .filter(|n| n.flow_rate > 0)
//         .count() as u32;
//
//     max_pressure(
//         important_nodes_lookup.get(&1u64).unwrap(),
//         30,
//         0,
//         0,
//         &important_nodes_lookup,
//         total_valves,
//     )
// }
//
fn convert_nodes(nodes: &Vec<Node<String>>) -> HashMap<u64, Node<u64>> {
    let nodes_mapping: HashMap<String, u64> = nodes.iter()
        .map(|n| n.name.clone())
        .sorted()
        .enumerate()
        .map(|(i, name)| (name, 1 << i))
        .collect();
    let nodes_lookup: HashMap<u64, Node<u64>> = nodes.iter()
        .map(|Node { name, neighbours, flow_rate }| {
            let node = Node::<u64> {
                name: nodes_mapping.get(name).unwrap().clone(),
                flow_rate: flow_rate.clone(),
                neighbours: neighbours.iter()
                    .map(|n| (1, nodes_mapping.get(&n.1).unwrap().clone()))
                    .collect_vec(),
            };
            (nodes_mapping.get(name).unwrap().clone(), node)
        })
        .collect();
    nodes_lookup
}
//
// #[cached(
// type = "SizedCache<(u64, i32, u64, i32), i32>",
// create = "{ SizedCache::with_size(100000000) }",
// convert = r#"{ (node.name, minutes_remaining, open_valves, total_flow_rate) }"#
// )]
// fn max_pressure(
//     node: &Node<u64>,
//     minutes_remaining: i32,
//     open_valves: u64,
//     total_flow_rate: i32,
//     nodes: &HashMap<u64, Node<u64>>,
//     total_valves: u32,
// ) -> i32 {
//     // let ten_millis = time::Duration::from_millis(200);
//     // thread::sleep(ten_millis);
//     // if minutes_remaining == 20 {
//     //     println!("node: {} minutes: {} valves: {:?} pressure: {}", node.name , minutes_remaining, open_valves, pressure_released);
//     // }
//
//
//
//     if minutes_remaining == 0 { return 0; }
//
//     let mut max = -1;
//
//     if open_valves.count_ones() == total_valves {
//         return total_flow_rate * minutes_remaining;
//     }
//
//     if node.flow_rate > 0 {
//         let open_valves = open_valves.clone();
//         let new_open_valves = open_valves | node.name;
//         let is_new_valve_opened = open_valves == new_open_valves;
//         let mut new_total_flow_rate = total_flow_rate;
//         if is_new_valve_opened { new_total_flow_rate += node.flow_rate }
//
//         let pressure = max_pressure(
//             node,
//             minutes_remaining - 1,
//             new_open_valves,
//             new_total_flow_rate,
//             nodes,
//             total_valves,
//         ) + total_flow_rate;
//         if max < pressure {
//             max = pressure
//         }
//     }
//
//     for (distance, neighbour) in &node.neighbours {
//         if neighbour | open_valves == open_valves {
//             continue
//         }
//         let neighbour_node = nodes.get(neighbour).unwrap();
//
//         let pressure = {
//             if distance.clone() > minutes_remaining {
//                 minutes_remaining * total_flow_rate
//             } else {
//                 max_pressure(
//                     neighbour_node,
//                     minutes_remaining - distance,
//                     open_valves,
//                     total_flow_rate,
//                     nodes,
//                     total_valves,
//                 ) + total_flow_rate * minutes_remaining
//             }
//         };
//         if max < pressure {
//             max = pressure
//         }
//     }
//
//     max
// }

#[derive(Clone, Debug, PartialEq)]
struct Node<T> {
    name: T,
    flow_rate: usize,
    neighbours: Vec<(usize, T)>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct SearchState {
    node: u64,
    open_valves: u64,
    current_flow: usize,
    elapsed_time: u64,
}

impl FromStr for Node<String> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words = s.split_whitespace().collect_vec();
        let name = words[1].to_string();
        let flow_rate: usize = words[4]
            .strip_prefix("rate=").unwrap()
            .strip_suffix(";").unwrap()
            .parse().unwrap();
        let neighbours = words[9..].iter()
            .map(|s| (1, s.trim_end_matches(",").to_string()))
            .collect_vec();
        Ok(Node { name, flow_rate, neighbours })
    }
}


#[cfg(test)]
mod tests {
    use crate::day16::*;

    #[test]
    fn parse_raw_input_to_node() {
        let raw_node = "Valve CC has flow rate=2; tunnels lead to valves DD, BB".to_string();
        let parsed_node = Node {
            name: "CC".to_string(),
            flow_rate: 2,
            neighbours: vec![(1, "DD".to_string()), (1, "BB".to_string())],
        };

        assert_eq!(raw_node.parse(), Ok(parsed_node));
    }

    #[test]
    fn should_solve_part_1_on_example() {
        let input: Vec<Node<String>> = vec![
            "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB".to_string(),
            "Valve BB has flow rate=13; tunnels lead to valves CC, AA".to_string(),
            "Valve CC has flow rate=2; tunnels lead to valves DD, BB".to_string(),
            "Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE".to_string(),
            "Valve EE has flow rate=3; tunnels lead to valves FF, DD".to_string(),
            "Valve FF has flow rate=0; tunnels lead to valves EE, GG".to_string(),
            "Valve GG has flow rate=0; tunnels lead to valves FF, HH".to_string(),
            "Valve HH has flow rate=22; tunnel leads to valve GG".to_string(),
            "Valve II has flow rate=0; tunnels lead to valves AA, JJ".to_string(),
            "Valve JJ has flow rate=21; tunnel leads to valve II".to_string(),
        ].iter().map(|s| s.parse::<Node<String>>().unwrap()).collect();
        assert_eq!(max_releasable_pressure(input), 1651);
    }
}
