use anyhow::Result;
use regex::Regex;
use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::time::Instant;

fn main() {
    println!("Hello, world!");
    let input = include_str!("../data.prod");
    let start = Instant::now();
    let output = part_1(input);
    println!("output: {output} in {:?}", Instant::now() - start);
}
const GAME_LENGTH: usize = 30;
const ELEPHANT_GAME_LENGTH: usize = 24;

fn part_2(input: &str) -> usize {
    let valves: Vec<Valve> = input.lines().map(|line| line.parse().unwrap()).collect();
    let graph = MeaningfulValveGraph::new(valves);
    let initial_game_state = ElephantGameState {
        human_turn: 1,
        elephant_turn: 1,
        score: 0,
        rate: 0,
        human_valve: "AA".to_string(),
        elephant_valve: "AA".to_string(),
        available_valves: graph
            .valves
            .clone()
            .into_iter()
            .map(|(key, _)| key)
            .collect(),
    };
    let mut future_game_states = vec![initial_game_state];
    let mut best_score = 0;
    while let Some(state) = future_game_states.pop() {
        //        println!("Beginning to evaluate state: {state:?}");
        // Evaluate sitting here and waiting state
        let remaining_turns = (ELEPHANT_GAME_LENGTH - state.turn()) + 1;
        let final_score = state.score + state.rate * remaining_turns;
        if final_score > best_score {
            best_score = final_score;
        }

        let mut new_states = vec![];

        if state.turn() == state.human_turn && state.turn() == state.elephant_turn {
            //Generate alternate states with the new human position and elephant turn
        } else if state.turn() == state.human_turn {
            // Just need to create a new state for the human
            new_states = state
                .available_valves
                .iter()
                .map(|valve_key| {
                    let human_destination = graph.valves.get(valve_key).unwrap();
                    let mut available_valves = state.available_valves.clone();
                    available_valves.remove(&human_destination.key);
                    o
                    let human_travel_time =
                        graph.shortest_path(&state.human_valve, &human_destination.key) + 1;
                    let human_turn = state.human_turn + human_travel_time;
                    let elephant_travel_time = state.turn() - state.elephant_turn;
                    let score = if human_turn <= state.elephant_turn {
                        state.score + state.rate * human_travel_time
                    } else {
                        state.score + state.rate * elephant_travel_time
                    };
                    let rate = if human_turn < state.elephant_turn {
                        state.rate + human_destination.flow_rate
                    } else {
                        state.rate + 
                    }
                    ElephantGameState {
                        human_turn: human_turn.clone(),
                        elephant_turn: state.elephant_turn,
                        score,
                    }
                })
                .collect()
        } else if state.turn() == state.elephant_turn {
            // Just need to create a new state for the elephant
        } else {
            unreachable!()
        }

        let new_states: Vec<ElephantGameState> = state
            .available_valves
            .iter()
            .flat_map(|human_key| {
                // consider human move first
                let human_destination = graph.valves.get(human_key).unwrap();
                let mut available_valves = state.available_valves.clone();
                available_valves.remove(&human_destination.key);
                let human_travel_time =
                    graph.shortest_path(&state.human_valve, &human_destination.key) + 1;
                let human_turn = state.human_turn + human_travel_time;
                available_valves
                    .iter()
                    .map(|elephant_key| {
                        let elephant_destination = graph.valves.get(elephant_key).unwrap();
                        let mut elephant_available_valves = available_valves.clone();
                        elephant_available_valves.remove(&elephant_destination.key);
                        let elephant_travel_time = graph
                            .shortest_path(&state.elephant_valve, &elephant_destination.key)
                            + 1;
                        let elephant_turn = state.elephant_turn + elephant_travel_time;
                        ElephantGameState {
                            human_turn: human_turn.clone(),
                            elephant_turn,
                            score: state.score + state.rate * min(human_turn, elephant_turn),
                            rate: state.rate + human_destination.flow_rate,
                            human_valve: human_destination.key.clone(),
                            elephant_valve: elephant_destination.key.clone(),
                            available_valves: elephant_available_valves,
                        }
                    })
                    .collect::<Vec<ElephantGameState>>()
            })
            .filter(|state| state.turn() <= ELEPHANT_GAME_LENGTH)
            .collect();

        // When adding a new state for consideration, we set the game state to be that we have
        // moved there and opened the valve, and we are on the first turn where that state will
        // give us value
        for new_state in new_states.into_iter() {
            future_game_states.push(new_state);
        }
    }

    best_score
}

fn part_1(input: &str) -> usize {
    let valves: Vec<Valve> = input.lines().map(|line| line.parse().unwrap()).collect();
    let graph = MeaningfulValveGraph::new(valves);
    let initial_game_state = GameState {
        turn: 1,
        score: 0,
        rate: 0,
        valve: "AA".to_string(),
        available_valves: graph
            .valves
            .clone()
            .into_iter()
            .map(|(key, _)| key)
            .collect(),
    };
    let mut future_game_states = vec![initial_game_state];
    let mut best_score = 0;
    while let Some(state) = future_game_states.pop() {
        //        println!("Beginning to evaluate state: {state:?}");
        // Evaluate sitting here and waiting state
        let remaining_turns = (GAME_LENGTH - state.turn) + 1;
        let final_score = state.score + state.rate * remaining_turns;
        if final_score > best_score {
            best_score = final_score;
        }
        let new_states: Vec<GameState> = state
            .available_valves
            .iter()
            .map(|key| {
                let destination = graph.valves.get(key).unwrap();
                let mut available_valves = state.available_valves.clone();
                available_valves.remove(&destination.key);
                let travel_time = graph.shortest_path(&state.valve, &destination.key) + 1;
                GameState {
                    turn: state.turn + travel_time,
                    score: state.score + state.rate * travel_time,
                    rate: state.rate + destination.flow_rate,
                    valve: destination.key.clone(),
                    available_valves,
                }
            })
            .filter(|state| state.turn <= GAME_LENGTH)
            .collect();

        // When adding a new state for consideration, we set the game state to be that we have
        // moved there and opened the valve, and we are on the first turn where that state will
        // give us value
        for new_state in new_states.into_iter() {
            future_game_states.push(new_state);
        }
    }

    best_score
}

#[derive(Debug)]
struct ElephantGameState {
    human_turn: usize,
    human_valve: String,
    elephant_turn: usize,
    elephant_valve: String,
    score: usize,
    rate: usize,
    available_valves: HashSet<String>,
}

impl ElephantGameState {
    fn turn(&self) -> usize {
        min(self.elephant_turn, self.human_turn)
    }
}

#[derive(Debug)]
struct GameState {
    turn: usize,
    score: usize,
    rate: usize,
    valve: String,
    available_valves: HashSet<String>,
}

#[derive(PartialEq, Debug)]
struct MeaningfulValveGraph {
    valves: HashMap<String, Valve>,
    shortest_paths: HashMap<String, HashMap<String, usize>>,
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

        let shortest_paths: HashMap<String, HashMap<String, usize>> = meaningful_valves
            .iter()
            .map(|valve| {
                let destinations: HashMap<String, usize> =
                    find_full_shortest_path_graph(&all_valves_map, valve.key.clone())
                        .into_iter()
                        .filter(|(key, _)| meaningful_valves.iter().any(|valve| valve.key == *key))
                        .collect();
                return (valve.key.clone(), destinations);
            })
            .collect();
        Self {
            valves: meaningful_valves
                .into_iter()
                .map(|valve| (valve.key.clone(), valve))
                .collect(),
            shortest_paths,
        }
    }
    fn shortest_path(&self, from: &String, to: &String) -> usize {
        *self
            .shortest_paths
            .get(from)
            .expect("Unable to find from")
            .get(to)
            .expect("Unable to find to")
    }
}

fn find_full_shortest_path_graph(
    all_valves: &HashMap<String, &Valve>,
    target_valve_key: String,
) -> HashMap<String, usize> {
    let mut shortest_paths: HashMap<String, usize> = all_valves
        .iter()
        .map(|(key, _)| (key.clone(), usize::MAX))
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
    distance_from_start: usize,
}

#[derive(Debug, Clone, PartialEq)]
struct Valve {
    key: String,
    flow_rate: usize,
    tunnels: HashSet<String>,
}

impl Valve {
    fn new(key: String, flow_rate: usize, tunnels: Vec<String>) -> Self {
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
        let flow_rate: usize = flow_regex
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

        let result = part_1(input);

        assert_eq!(result, 1651);
    }

    #[test]
    fn part_2_given() {
        let input = include_str!("../data.test");

        let result = part_2(input);

        assert_eq!(result, 1707);
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
        let a_to_b = meaningful_graph.shortest_path(&a_string, &b_string);
        let b_to_a = meaningful_graph.shortest_path(&b_string, &a_string);

        assert_eq!(meaningful_graph.valves, expected_graph_valves);
        assert_eq!(a_to_b, expected_a_b_path);
        assert_eq!(b_to_a, expected_a_b_path);
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
        let a_to_b = meaningful_graph.shortest_path(&a_string, &b_string);
        let b_to_a = meaningful_graph.shortest_path(&b_string, &a_string);

        assert_eq!(meaningful_graph.valves, expected_graph_valves);
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
        let a_to_e = meaningful_graph.shortest_path(&a_string, &e_string);
        let a_to_b = meaningful_graph.shortest_path(&a_string, &b_string);
        let b_to_e = meaningful_graph.shortest_path(&b_string, &e_string);

        assert_eq!(meaningful_graph.valves, expected_graph_valves);
        assert_eq!(a_to_e, a_e_distance);
        assert_eq!(a_to_b, a_b_distance);
        assert_eq!(b_to_e, e_b_distance);
    }
}
