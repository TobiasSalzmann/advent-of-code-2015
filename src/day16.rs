use std::collections::HashMap;
use std::str::FromStr;
use itertools::Itertools;
use pathfinding::prelude::dijkstra;
use crate::util;

pub fn main() {
    let input: Vec<Node<String>> = util::parse_from_strings("resources/day16.txt");

    println!("Day 16, Part 1: {:?}", max_releasable_pressure(input.clone()));
    println!("Day 16, Part 2: {:?}", max_releasable_pressure_with_elephant(input.clone()));
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

    for (_, neighbour) in &node.neighbours {
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

fn max_releasable_pressure_with_elephant(nodes: Vec<Node<String>>) -> i64 {
    let node_lookup = convert_nodes(&nodes);
    let maximum_flow = node_lookup.values().map(|n| n.flow_rate).sum();
    let start = SearchStateWithElephant {
        node: 1,
        elephant_node: 1,
        open_valves: 0,
        current_flow: 0,
        elapsed_time: 0,
    };
    let result: (Vec<SearchStateWithElephant>, usize) = dijkstra(
        &start,
        |n| successors_with_elephant(n, &node_lookup, maximum_flow),
        |n| n.elapsed_time == 26,
    ).unwrap();

    (result.1 as i64 - (26 * maximum_flow as i64)).abs()
}

fn successors_with_elephant(search_state: &SearchStateWithElephant, node_lookup: &HashMap<u64, Node<u64>>, maximum_flow: usize) -> Vec<(SearchStateWithElephant, usize)> {
    let mut succs = vec![];
    let node = node_lookup.get(&search_state.node).unwrap();
    let elephant_node = node_lookup.get(&search_state.elephant_node).unwrap();
    let cost = maximum_flow - search_state.current_flow;

    let valve_already_open = (search_state.open_valves | search_state.node) == search_state.open_valves;
    let elephant_valve_already_open = (search_state.open_valves | search_state.elephant_node) == search_state.open_valves;

    // elf and elephant open valve
    if node.flow_rate > 0 && !valve_already_open && elephant_node.flow_rate > 0 && !elephant_valve_already_open && search_state.node != search_state.elephant_node {
        let state = SearchStateWithElephant {
            node: search_state.node,
            elephant_node: search_state.elephant_node,
            open_valves: (search_state.open_valves | search_state.node) | search_state.elephant_node,
            current_flow: search_state.current_flow + node.flow_rate + elephant_node.flow_rate,
            elapsed_time: search_state.elapsed_time + 1,
        };
        succs.push((state, cost));
    }

    // elf opens valve, elephant moves
    if node.flow_rate > 0 && !valve_already_open {
        for (_, neighbour) in &elephant_node.neighbours {
            let state = SearchStateWithElephant {
                node: search_state.node,
                elephant_node: neighbour.clone(),
                open_valves: search_state.open_valves | search_state.node,
                current_flow: search_state.current_flow + node.flow_rate,
                elapsed_time: search_state.elapsed_time + 1,
            };
            succs.push((state, cost));
        }
    }

    // elf moves, elephant opens valve
    if elephant_node.flow_rate > 0 && !elephant_valve_already_open {
        for (_, neighbour) in &node.neighbours {
            let state = SearchStateWithElephant {
                node: neighbour.clone(),
                elephant_node: search_state.elephant_node,
                open_valves: search_state.open_valves | search_state.elephant_node,
                current_flow: search_state.current_flow + elephant_node.flow_rate,
                elapsed_time: search_state.elapsed_time + 1,
            };
            succs.push((state, cost));
        }
    }

    // both move

    for (_, neighbour) in &node.neighbours {
        for (_, elephant_neighbour) in &elephant_node.neighbours {
            let state = SearchStateWithElephant {
                node: neighbour.clone(),
                elephant_node: elephant_neighbour.clone(),
                open_valves: search_state.open_valves,
                current_flow: search_state.current_flow,
                elapsed_time: search_state.elapsed_time + 1,
            };
            succs.push((state, cost));
        }
    }

    succs
}

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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct SearchStateWithElephant {
    node: u64,
    elephant_node: u64,
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

    #[test]
    fn should_solve_part_2_on_example() {
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
        assert_eq!(max_releasable_pressure_with_elephant(input), 1707);
    }
}
