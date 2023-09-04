use anyhow::Result;
use core::cmp::Ordering;
use regex::Regex;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::str::FromStr;
use std::time::Instant;

fn main() {
    println!("Hello, world!");
    let input = include_str!("../data.prod");
    let start = Instant::now();
    let output = solve_game(input, 30, 1);
    println!("output: {output} in {:?}", Instant::now() - start);
}

fn solve_game(input: &str, game_length: u8, agent_count: u8) -> u64 {
    let valves: Vec<Valve> = input.lines().map(|line| line.parse().unwrap()).collect();
    let graph = MeaningfulValveGraph::new(valves);
    //println!("Generated graph of {graph:#?}");
    let mut future_game_states = BinaryHeap::new();
    let starting_game_state = GameState::new(&graph, game_length, agent_count);
    future_game_states.push(Priority(
        starting_game_state.very_optimistic_score(&graph, game_length),
        starting_game_state,
    ));
    let mut best_score = 0;
    while let Some(priority_state) = future_game_states.pop() {
        let mut state = priority_state.1;
        let upper_limit = priority_state.0;
    }

    best_score
}

#[derive(Debug, Clone)]
struct GameState {
    score: u64,
    rate: u64,
    closed_valves: ClosedValves,
    agents: Vec<Agent>,
}

#[derive(Clone, PartialEq, Debug)]
struct ClosedValves {
    data: u64,
}

impl ClosedValves {
    fn new(valves: &Vec<Valve>) -> Self {
        let mut data = 0u64;
        for i in 0..valves.len() {
            data = data | 1 << i;
        }
        Self { data }
    }
    #[inline(always)]
    fn close_valve(&self, valve_index: usize) -> Self {
        let data = self.data & !(1 << valve_index);
        Self { data }
    }
    #[inline(always)]
    fn valve_closed(&self, valve_index: u64) -> bool {
        (self.data & 1 << valve_index) != 0
    }
}

impl GameState {
    fn new(graph: &MeaningfulValveGraph, game_length: u8, agent_count: u8) -> Self {
        GameState {
            score: 0,
            rate: 0,
            closed_valves: ClosedValves::new(&graph.valves),
            agents: (0..agent_count)
                .map(|id| Agent::new(id.try_into().unwrap()))
                .collect(),
        }
    }
    fn sort_agents(&mut self) {
        self.agents.sort_unstable_by(|a, b| a.turn.cmp(&b.turn))
    }
    fn very_optimistic_score(&self, graph: &MeaningfulValveGraph, game_length: u8) -> u64 {
        let earliest_time = self.agents[0].turn;
        let mut remaining_turns = game_length - earliest_time;

        self.score + 0
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Agent {
    id: u8,
    turn: u8,
    valve: usize,
}

impl Agent {
    fn new(id: u8) -> Self {
        Self {
            id,
            turn: 1,
            valve: 0,
        }
    }
    fn valve(&self, graph: &MeaningfulValveGraph) -> String {
        graph.get_valve_key(self.valve).unwrap().to_string()
    }
}

#[derive(PartialEq, Debug)]
struct MeaningfulValveGraph {
    valves: Vec<Valve>,
    shortest_paths: Vec<Vec<u8>>,
}

impl MeaningfulValveGraph {
    fn new(valves: Vec<Valve>) -> Self {
        let all_valves_map: HashMap<String, &Valve> = valves
            .iter()
            .map(|valve| (valve.key.clone(), valve))
            .collect();
        //        println!("{all_valves_map:?}");

        let meaningful_valves: Vec<Valve> = valves
            .clone()
            .into_iter()
            .filter(|valve| valve.flow_rate > 0 || valve.key == "AA")
            .collect();
        //       println!("meaningful valves: {meaningful_valves:?}");

        let shortest_paths: HashMap<String, HashMap<String, u8>> = meaningful_valves
            .iter()
            .map(|valve| {
                let destinations: HashMap<String, u8> =
                    find_full_shortest_path_graph(&all_valves_map, valve.key.clone())
                        .into_iter()
                        .filter(|(key, _)| meaningful_valves.iter().any(|valve| valve.key == *key))
                        .collect();
                return (valve.key.clone(), destinations);
            })
            .collect();

        let mut shortest_paths_vec =
            vec![vec![u8::MAX; meaningful_valves.len()]; meaningful_valves.len()];

        for (from, destinations) in shortest_paths.iter() {
            let from_index = meaningful_valves
                .iter()
                .position(|valve| valve.key == *from)
                .unwrap();
            for (to, distance) in destinations.iter() {
                let to_index = meaningful_valves
                    .iter()
                    .position(|valve| valve.key == *to)
                    .unwrap();
                shortest_paths_vec[from_index][to_index] = *distance;
            }
        }

        Self {
            valves: meaningful_valves,
            shortest_paths: shortest_paths_vec,
        }
    }
    fn get_valve_key(&self, valve_index: usize) -> Option<&str> {
        Some(&self.valves.get(valve_index)?.key)
    }
    fn get_valve_index(&self, valve_key: &str) -> Option<usize> {
        self.valves.iter().position(|valve| valve.key == valve_key)
    }
    fn valves_hashset(&self) -> HashMap<String, Valve> {
        self.valves
            .clone()
            .into_iter()
            .map(|valve| (valve.key.clone(), valve))
            .collect()
    }
    fn shortest_path_key(&self, from: &String, to: &String) -> u8 {
        let from_index = self.get_valve_index(from).unwrap();
        let to_index = self.get_valve_index(to).unwrap();
        self.shortest_path(from_index, to_index)
    }
    #[inline(always)]
    fn shortest_path(&self, from: usize, to: usize) -> u8 {
        self.shortest_paths[from][to]
    }
    fn move_agent_to_destination_key(&self, agent: &Agent, destination_key: &String) -> Agent {
        let destination_index = self.get_valve_index(destination_key).unwrap();
        self.move_agent_to_destination(agent, destination_index)
    }
    fn move_agent_to_destination(&self, agent: &Agent, destination_index: usize) -> Agent {
        let destination = &self.valves[destination_index];

        let travel_time = self.shortest_path(agent.valve, destination_index) + 1;

        Agent {
            id: agent.id,
            turn: agent.turn + travel_time,
            valve: destination_index,
        }
    }
}

fn find_full_shortest_path_graph(
    all_valves: &HashMap<String, &Valve>,
    target_valve_key: String,
) -> HashMap<String, u8> {
    let mut shortest_paths: HashMap<String, u8> = all_valves
        .iter()
        .map(|(key, _)| (key.clone(), u8::MAX))
        .collect();
    shortest_paths.insert(target_valve_key.clone(), 0);
    let mut nodes_to_consider = vec![ShortestPathNode {
        key: target_valve_key.clone(),
        distance_from_start: 0,
    }];
    while let Some(node) = nodes_to_consider.pop() {
        let all_neighbors = all_valves.get(&node.key).unwrap().tunnels.clone();
        let all_neighbors_distance: Vec<ShortestPathNode> = all_neighbors
            .clone()
            .into_iter()
            .map(|key| {
                let distance = node.distance_from_start + 1;
                ShortestPathNode {
                    key,
                    distance_from_start: distance,
                }
            })
            .collect();
        let shortest_neighbors_distance: Vec<ShortestPathNode> = all_neighbors_distance
            .into_iter()
            .filter(|node| {
                let current_shortest_path = shortest_paths.get(&node.key).unwrap();
                node.distance_from_start < *current_shortest_path
            })
            .collect();
        for node in shortest_neighbors_distance.into_iter() {
            shortest_paths.insert(node.key.to_string(), node.distance_from_start);
            nodes_to_consider.push(node);
        }
    }

    shortest_paths
}

struct ShortestPathNode {
    key: String,
    distance_from_start: u8,
}

#[derive(Debug, Clone, PartialEq)]
struct Valve {
    key: String,
    flow_rate: u8,
    tunnels: HashSet<String>,
}

impl Valve {
    fn new(key: String, flow_rate: u8, tunnels: Vec<String>) -> Self {
        Self {
            key,
            flow_rate,
            tunnels: tunnels.into_iter().collect(),
        }
    }
}

impl FromStr for Valve {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (flow, tunnels_section) = s.split_once(";").unwrap();
        let key_regex: Regex = Regex::new(r"Valve (\S*) has").unwrap();
        let captures = key_regex.captures(flow).unwrap();
        let key = String::from(captures.get(1).unwrap().as_str());
        let flow_regex: Regex = Regex::new(r"flow rate=(\d*)").unwrap();
        let flow_rate = flow_regex
            .captures(flow)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse()
            .unwrap();
        let tunnel_regex = Regex::new(r"valve\S*(.*)$").unwrap();
        let tunnels = tunnel_regex
            .captures(tunnels_section)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str();
        let tunnels: HashSet<String> = tunnels
            .split(",")
            .map(|val| String::from(val.trim()))
            .collect();

        return Ok(Self {
            key,
            flow_rate,
            tunnels,
        });
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn part_1_given() {
        let input = include_str!("../data.test");

        let result = solve_game(input, 30, 1);

        assert_eq!(result, 1651);
    }

    #[test]
    #[ignore]
    fn part_2_given() {
        let input = include_str!("../data.test");

        let result = solve_game(input, 26, 2);

        assert_eq!(result, 1707);
    }

    #[test]
    #[ignore]
    fn part_1_full() {
        let input = include_str!("../data.prod");

        let result = solve_game(input, 30, 1);

        assert_eq!(result, 2253);
    }

    #[test]
    fn valve_key_parse() {
        let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB";

        let valve: Valve = input.parse().unwrap();
        let valve_key = valve.key;

        assert_eq!(valve_key, String::from("AA"));
    }

    #[test]
    fn valve_key_parse_3_char() {
        let input = "Valve ABC has flow rate=0; tunnels lead to valves DD, II, BB";

        let valve: Valve = input.parse().unwrap();
        let valve_key = valve.key;

        assert_eq!(valve_key, String::from("ABC"));
    }

    #[test]
    fn parse_flow_rate() {
        let input = "Valve ABC has flow rate=100; tunnels lead to valves DD, II, BB";

        let valve: Valve = input.parse().unwrap();
        let flow_rate = valve.flow_rate;

        assert_eq!(flow_rate, 100);
    }

    #[test]
    fn parse_tunnels() {
        let input = "Valve ABC has flow rate=100; tunnels lead to valves DD, II, BB";
        let expected_tunnels: HashSet<String> =
            vec![String::from("DD"), String::from("II"), String::from("BB")]
                .into_iter()
                .collect();

        let valve: Valve = input.parse().unwrap();
        let tunnels = valve.tunnels;

        assert_eq!(tunnels, expected_tunnels);
    }

    #[test]
    fn parse_tunnel() {
        let input = "Valve ABC has flow rate=100; tunnels lead to valve DD";
        let expected_tunnels: HashSet<String> = vec![String::from("DD")].into_iter().collect();

        let valve: Valve = input.parse().unwrap();
        let tunnels = valve.tunnels;

        assert_eq!(tunnels, expected_tunnels);
    }

    #[test]
    fn meaningful_graph_basic() {
        // a - b
        let a_string = "AA".to_string();
        let b_string = "BB".to_string();
        let a = Valve::new(a_string.clone(), 1, vec![b_string.clone()]);
        let b = Valve::new(b_string.clone(), 1, vec![a_string.clone()]);
        let valves = vec![a.clone(), b.clone()];
        let expected_graph_valves: HashMap<String, Valve> =
            vec![(a_string.clone(), a.clone()), (b_string.clone(), b.clone())]
                .into_iter()
                .collect();
        let expected_a_b_path = 1;

        let meaningful_graph: MeaningfulValveGraph = MeaningfulValveGraph::new(valves);
        let a_to_b = meaningful_graph.shortest_path_key(&a_string, &b_string);
        let b_to_a = meaningful_graph.shortest_path_key(&b_string, &a_string);

        assert_eq!(meaningful_graph.valves_hashset(), expected_graph_valves);
        assert_eq!(a_to_b, expected_a_b_path);
        assert_eq!(b_to_a, expected_a_b_path);
    }

    #[test]
    fn meaningful_graph_index_to_string() {
        let a_string = "AA".to_string();
        let a = Valve::new(a_string.clone(), 1, vec![]);

        let meaningful_graph: MeaningfulValveGraph = MeaningfulValveGraph::new(vec![a]);

        let a_index = meaningful_graph.get_valve_index(&a_string).unwrap();
        let a_string_got = meaningful_graph.get_valve_key(a_index).unwrap();

        assert_eq!(a_index, 0);
        assert_eq!(a_string_got, a_string);
    }

    #[test]
    fn meaningful_graph_intermediate_node() {
        // a - b -c
        let a_string = "AA".to_string();
        let b_string = "BB".to_string();
        let c_string = "CC".to_string();
        let a = Valve::new(a_string.clone(), 1, vec![c_string.clone()]);
        let b = Valve::new(b_string.clone(), 1, vec![c_string.clone()]);
        let c = Valve::new(
            c_string.clone(),
            0,
            vec![a_string.clone(), b_string.clone()],
        );
        let valves = vec![a.clone(), b.clone(), c.clone()];
        let expected_graph_valves: HashMap<String, Valve> =
            vec![(a_string.clone(), a.clone()), (b_string.clone(), b.clone())]
                .into_iter()
                .collect();
        let expected_a_b_path = 2;

        let meaningful_graph: MeaningfulValveGraph = MeaningfulValveGraph::new(valves);
        let a_to_b = meaningful_graph.shortest_path_key(&a_string, &b_string);
        let b_to_a = meaningful_graph.shortest_path_key(&b_string, &a_string);

        assert_eq!(meaningful_graph.valves_hashset(), expected_graph_valves);
        assert_eq!(a_to_b, expected_a_b_path);
        assert_eq!(b_to_a, expected_a_b_path);
    }

    #[test]
    fn meaningful_graph_alternate_path() {
        //     e
        //     |
        // a - c - d - b
        //  \-----/
        let a_string = "AA".to_string();
        let b_string = "BB".to_string();
        let c_string = "CC".to_string();
        let d_string = "DD".to_string();
        let e_string = "EE".to_string();
        let a = Valve::new(
            a_string.clone(),
            1,
            vec![c_string.clone(), d_string.clone()],
        );
        let b = Valve::new(b_string.clone(), 1, vec![d_string.clone()]);
        let c = Valve::new(
            c_string.clone(),
            0,
            vec![a_string.clone(), d_string.clone(), e_string.clone()],
        );
        let d = Valve::new(
            d_string.clone(),
            0,
            vec![a_string.clone(), c_string.clone(), b_string.clone()],
        );
        let e = Valve::new(e_string.clone(), 1, vec![c_string.clone()]);
        let valves = vec![a.clone(), b.clone(), c.clone(), d.clone(), e.clone()];
        let expected_graph_valves: HashMap<String, Valve> = vec![
            (a_string.clone(), a.clone()),
            (b_string.clone(), b.clone()),
            (e_string.clone(), e.clone()),
        ]
        .into_iter()
        .collect();
        let a_e_distance = 2;
        let a_b_distance = 2;
        let e_b_distance = 3;

        let meaningful_graph: MeaningfulValveGraph = MeaningfulValveGraph::new(valves);
        let a_to_e = meaningful_graph.shortest_path_key(&a_string, &e_string);
        let a_to_b = meaningful_graph.shortest_path_key(&a_string, &b_string);
        let b_to_e = meaningful_graph.shortest_path_key(&b_string, &e_string);

        assert_eq!(meaningful_graph.valves_hashset(), expected_graph_valves);
        assert_eq!(a_to_e, a_e_distance);
        assert_eq!(a_to_b, a_b_distance);
        assert_eq!(b_to_e, e_b_distance);
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Priority<P, T>(pub P, pub T);

impl<P: Ord + Eq, T> Ord for Priority<P, T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl<P: Ord + Eq, T> PartialOrd for Priority<P, T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<P: Eq, T> PartialEq for Priority<P, T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<P: Eq, T> Eq for Priority<P, T> {}
