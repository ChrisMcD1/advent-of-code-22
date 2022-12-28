use anyhow::Result;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::time::Instant;

fn main() {
    println!("Hello, world!");
    let input = include_str!("../data.prod");
    let start = Instant::now();
    let output = solve_game(input, 26, 2);
    println!("output: {output} in {:?}", Instant::now() - start);
}

fn solve_game(input: &str, game_length: usize, agent_count: usize) -> usize {
    let valves: Vec<Valve> = input.lines().map(|line| line.parse().unwrap()).collect();
    let graph = MeaningfulValveGraph::new(valves);
    println!("Generated graph of {graph:#?}");
    let mut future_game_states = vec![GameState::new(&graph, game_length, agent_count)];
    let mut best_score = 0;
    while let Some(mut state) = future_game_states.pop() {
        let last_turn = state.turn;
        state.turn = state.calculate_min_turn();
        state.update_score_since_last_turn(last_turn);
        state.open_all_active_valves(&graph);

        let final_score = state.score_without_opening_more_valves();
        if final_score > best_score {
            println!("Found a better end score for: {state:#?}. Score {final_score}");
            best_score = final_score;
        }

        let new_states = generate_game_states(&graph, &state);

        for new_state in new_states.into_iter() {
            future_game_states.push(new_state);
        }
    }

    best_score
}

fn generate_game_states(graph: &MeaningfulValveGraph, state: &GameState) -> Vec<GameState> {
    //println!("Generating for {state:#?}");

    let new_states: Vec<GameState> = state
        .active_agents()
        .iter()
        .flat_map(|agent| {
            let new_states = state.generate_moves_for_agent(graph, agent);

            let mut other_agents_moved_states: Vec<GameState> = new_states
                .iter()
                .flat_map(|new_state| generate_game_states(graph, &new_state))
                .collect();
            for state in new_states.into_iter() {
                other_agents_moved_states.push(state);
            }

            other_agents_moved_states
        })
        .collect();
    //println!("Generated new_states of {new_states:#?}");
    new_states
        .into_iter()
        .filter(|state| !state.any_agents_going_to_same_place())
        .filter(|state| state.all_agents_moving_if_possible())
        .collect()
}
#[derive(Debug, Clone)]
struct GameState {
    turn: usize,
    game_length: usize,
    score: usize,
    rate: usize,
    available_valves: HashSet<String>,
    agents: Vec<Agent>,
    open_valves: HashSet<String>,
    history: Vec<GameState>,
}

impl GameState {
    fn new(graph: &MeaningfulValveGraph, game_length: usize, agent_count: usize) -> Self {
        GameState {
            turn: 1,
            game_length,
            score: 0,
            rate: 0,
            history: vec![],
            open_valves: HashSet::new(),
            available_valves: graph
                .valves
                .clone()
                .into_iter()
                .map(|(key, _)| key)
                .filter(|key| key != "AA")
                .collect(),
            agents: (0..agent_count).map(|id| Agent::new(id)).collect(),
        }
    }

    fn all_agents_moving_if_possible(&self) -> bool {
        let agent_moving_count = self
            .agents
            .iter()
            .filter(|agent| agent.wakeup_turn > self.turn)
            .count();
        self.available_valves.len() == agent_moving_count || agent_moving_count == self.agents.len()
    }
    fn score_without_opening_more_valves(&self) -> usize {
        let remaining_turns = (self.game_length - self.turn) + 1;
        let final_score = self.score + self.rate * remaining_turns;
        final_score
    }
    fn open_all_active_valves(&mut self, graph: &MeaningfulValveGraph) {
        self.rate = self.rate
            + self
                .active_agents()
                .iter()
                .filter(|agent| !self.open_valves.contains(&agent.valve))
                .map(|agent| graph.valves.get(&agent.valve).unwrap().flow_rate)
                .sum::<usize>();

        for agent in self.active_agents().iter() {
            self.open_valves.insert(agent.valve.clone());
        }
    }
    fn update_score_since_last_turn(&mut self, last_turn: usize) {
        self.score = self.score + self.rate * (self.turn - last_turn);
    }
    fn calculate_min_turn(&self) -> usize {
        self.agents
            .iter()
            .map(|agent| agent.wakeup_turn)
            .min()
            .expect("Agents must be initialized")
    }
    fn any_agents_going_to_same_place(&self) -> bool {
        self.agents.iter().any(|agent| {
            let mut other_agents = self.agents.iter().filter(|other| *other != agent);
            other_agents.any(|other| other.valve == agent.valve)
        })
    }
    fn active_agents(&self) -> Vec<Agent> {
        self.agents
            .clone()
            .into_iter()
            .filter(|agent| agent.wakeup_turn == self.turn)
            .collect()
    }
    fn move_agent_and_generate_new_state(&self, agent: Agent) -> Self {
        let mut available_valves = self.available_valves.clone();
        available_valves.remove(&agent.valve);

        let other_agents: Vec<Agent> = self
            .agents
            .clone()
            .into_iter()
            .filter(|existing_agent| existing_agent.id != agent.id)
            .collect();

        let mut agents = other_agents.clone();
        agents.push(agent);

        let mut new_history = self.history.clone();
        let mut history_object = self.clone();
        history_object.history = vec![];
        new_history.push(history_object);
        let new_state = GameState {
            turn: self.turn,
            game_length: self.game_length,
            score: self.score,
            rate: self.rate,
            available_valves,
            open_valves: self.open_valves.clone(),
            agents,
            history: new_history,
        };
        new_state
    }
    fn generate_moves_for_agent(
        &self,
        graph: &MeaningfulValveGraph,
        agent: &Agent,
    ) -> Vec<GameState> {
        let new_states: Vec<GameState> = self
            .available_valves
            .iter()
            .map(|destination_key| {
                let new_agent = graph.move_agent_to_destination(agent, destination_key);

                self.move_agent_and_generate_new_state(new_agent)
            })
            .filter(|state| state.calculate_min_turn() <= state.game_length)
            .collect();

        new_states
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Agent {
    id: usize,
    wakeup_turn: usize,
    valve: String,
}

impl Agent {
    fn new(id: usize) -> Self {
        Self {
            id,
            wakeup_turn: 1,
            valve: "AA".to_string(),
        }
    }
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
    fn move_agent_to_destination(&self, agent: &Agent, destination_key: &String) -> Agent {
        let destination = self.valves.get(destination_key).unwrap();

        let travel_time = self.shortest_path(&agent.valve, &destination.key) + 1;

        Agent {
            id: agent.id,
            wakeup_turn: agent.wakeup_turn + travel_time,
            valve: destination.key.clone(),
        }
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

        let result = solve_game(input, 30, 1);

        assert_eq!(result, 1651);
    }

    #[test]
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
