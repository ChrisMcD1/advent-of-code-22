use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::ops::Sub;
use std::str::FromStr;
use std::time::Instant;

fn main() {
    println!("Hello, world!");
    let input = include_str!("../input.dev");
    let start = Instant::now();
    let output = part_2(input);
    let elapsed = Instant::now() - start;
    println!("Output is {:?}", output);
    println!("Took {:?} to run", elapsed);
}

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
struct Blueprint {
    id: usize,
    ore_robot_cost: Resources,
    clay_robot_cost: Resources,
    obsidian_robot_cost: Resources,
    geode_robot_cost: Resources,
}

impl Blueprint {
    fn max_cost(&self) -> Resources {
        return Resources {
            ore: *vec![
                self.ore_robot_cost.ore,
                self.clay_robot_cost.ore,
                self.obsidian_robot_cost.ore,
                self.geode_robot_cost.ore,
            ]
            .iter()
            .max()
            .unwrap(),
            clay: *vec![
                self.ore_robot_cost.clay,
                self.clay_robot_cost.clay,
                self.obsidian_robot_cost.clay,
                self.geode_robot_cost.clay,
            ]
            .iter()
            .max()
            .unwrap(),
            obsidian: *vec![
                self.ore_robot_cost.obsidian,
                self.clay_robot_cost.obsidian,
                self.obsidian_robot_cost.obsidian,
                self.geode_robot_cost.obsidian,
            ]
            .iter()
            .max()
            .unwrap(),
            geode: 0,
        };
    }
}

impl FromStr for Blueprint {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let blueprint_regex: Regex = Regex::new(
            r"Blueprint (\d*): Each ore robot costs (\d*) ore\. Each clay robot costs (\d*) ore\. Each obsidian robot costs (\d*) ore and (\d*) clay\. Each geode robot costs (\d*) ore and (\d*) obsidian\."
        ).unwrap();
        let captures = blueprint_regex.captures(s).unwrap();
        let id = captures[1].parse().unwrap();
        let ore_robot_ore_cost = captures[2].parse().unwrap();
        let clay_robot_ore_cost = captures[3].parse().unwrap();
        let obsidian_robot_ore_cost = captures[4].parse().unwrap();
        let obsidian_robot_clay_cost = captures[5].parse().unwrap();
        let geode_robot_ore_cost = captures[6].parse().unwrap();
        let geode_robot_obisidan_cost = captures[7].parse().unwrap();
        let ore_robot_cost = Resources::new(ore_robot_ore_cost, 0, 0, 0);
        let clay_robot_cost = Resources::new(clay_robot_ore_cost, 0, 0, 0);
        let obsidian_robot_cost =
            Resources::new(obsidian_robot_ore_cost, obsidian_robot_clay_cost, 0, 0);
        let geode_robot_cost =
            Resources::new(geode_robot_ore_cost, 0, geode_robot_obisidan_cost, 0);
        return Ok(Blueprint {
            id,
            ore_robot_cost,
            clay_robot_cost,
            obsidian_robot_cost,
            geode_robot_cost,
        });
    }
}

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone, Hash, Eq)]
struct RobotCount {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone, Hash, Eq)]
struct Resources {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

impl Sub for Resources {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            ore: self.ore - rhs.ore,
            clay: self.clay - rhs.clay,
            geode: self.geode - rhs.geode,
            obsidian: self.obsidian - rhs.obsidian,
        }
    }
}

impl Resources {
    fn greater_or_equal_than(&self, other: &Self) -> bool {
        return self.ore >= other.ore
            && self.clay >= other.clay
            && self.obsidian >= other.obsidian
            && self.geode >= other.geode;
    }
    fn new(ore: usize, clay: usize, obsidian: usize, geode: usize) -> Self {
        return Self {
            ore,
            clay,
            obsidian,
            geode,
        };
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Hash, Eq)]
struct GameState {
    move_number: usize,
    robot_count: Resources,
    resources: Resources,
}

impl GameState {
    fn new() -> Self {
        return Self {
            move_number: 0,
            resources: Resources {
                ore: 0,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            robot_count: Resources {
                ore: 1,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
        };
    }
    fn is_objectively_better_than(&self, other: &Self) -> bool {
        self.move_number <= other.move_number
            && self.resources.greater_or_equal_than(&other.resources)
            && self.robot_count.greater_or_equal_than(&other.robot_count)
    }
    fn score(&self, blueprint: &Blueprint, max_move_count: usize) -> u64 {
        return (max_move_count - self.move_number) as u64
            * (self.robot_count.geode * 1000
                + self.robot_count.obsidian * 1000 / blueprint.geode_robot_cost.obsidian)
                as u64;
    }
    fn collect_resources(&mut self) {
        self.resources.ore = self.resources.ore + self.robot_count.ore;
        self.resources.clay = self.resources.clay + self.robot_count.clay;
        self.resources.obsidian = self.resources.obsidian + self.robot_count.obsidian;
        self.resources.geode = self.resources.geode + self.robot_count.geode;
    }
    fn build_ore_robot_if_possible(mut self, blueprint: &Blueprint) -> Option<Self> {
        if !self
            .resources
            .greater_or_equal_than(&blueprint.ore_robot_cost)
        {
            return None;
        }
        if self.robot_count.ore == blueprint.max_cost().ore {
            //            println!("Skipping build ore robot because we have {:?} ore robots and only will ever need {:?}", self.robot_count.ore, blueprint.max_cost().ore);
            return None;
        }
        self.resources = self.resources - blueprint.ore_robot_cost;
        self.collect_resources();
        self.robot_count.ore = self.robot_count.ore + 1;
        return Some(self);
    }
    fn build_clay_robot_if_possible(mut self, blueprint: &Blueprint) -> Option<Self> {
        if !self
            .resources
            .greater_or_equal_than(&blueprint.clay_robot_cost)
        {
            return None;
        }
        if self.robot_count.clay == blueprint.max_cost().clay {
            //           println!("Skipping build clay robot because we have {:?} clay robots and only will ever need {:?}", self.robot_count.clay, blueprint.max_cost().clay);
            return None;
        }
        self.resources = self.resources - blueprint.clay_robot_cost;
        self.collect_resources();
        self.robot_count.clay = self.robot_count.clay + 1;
        return Some(self);
    }
    fn build_obsidian_robot_if_possible(mut self, blueprint: &Blueprint) -> Option<Self> {
        if !self
            .resources
            .greater_or_equal_than(&blueprint.obsidian_robot_cost)
        {
            return None;
        }
        if self.robot_count.obsidian == blueprint.max_cost().obsidian {
            //          println!("Skipping build obsidian robot because we have {:?} obsidian robots and only will ever need {:?}", self.robot_count.obsidian, blueprint.max_cost().obsidian);
            return None;
        }
        self.resources = self.resources - blueprint.obsidian_robot_cost;
        self.collect_resources();
        self.robot_count.obsidian = self.robot_count.obsidian + 1;
        return Some(self);
    }
    fn build_geode_robot_if_possible(mut self, blueprint: &Blueprint) -> Option<Self> {
        if !self
            .resources
            .greater_or_equal_than(&blueprint.geode_robot_cost)
        {
            return None;
        }
        self.resources = self.resources - blueprint.geode_robot_cost;
        self.collect_resources();
        self.robot_count.geode = self.robot_count.geode + 1;
        return Some(self);
    }
    fn can_buy_any_robot(&self, blueprint: &Blueprint) -> bool {
        self.resources.greater_or_equal_than(&blueprint.max_cost())
    }
    fn generate_all_next_moves(mut self, blueprint: &Blueprint) -> Vec<Self> {
        self.move_number = self.move_number + 1;
        let mut possible_moves = Vec::new();
        if !self.can_buy_any_robot(blueprint) || self.moves_left(MAX_MOVE_COUNT) == 0 {
            let mut do_nothing = self.clone();
            do_nothing.collect_resources();
            possible_moves.push(Some(do_nothing));
        }

        if self.moves_left(MAX_MOVE_COUNT) != 0 {
            let build_geode = self.clone().build_geode_robot_if_possible(blueprint);
            possible_moves.push(build_geode);
            possible_moves.push(self.clone().build_obsidian_robot_if_possible(blueprint));
            possible_moves.push(self.clone().build_clay_robot_if_possible(blueprint));
            possible_moves.push(self.clone().build_ore_robot_if_possible(blueprint));
        }
        let next_moves: Vec<Self> = possible_moves.into_iter().flatten().collect();
        return next_moves;
    }
    fn moves_left(&self, max_move_count: usize) -> usize {
        max_move_count - self.move_number
    }
    fn maximum_possible_geode_at_end(&self, max_move_count: usize) -> usize {
        let moves_left = self.moves_left(max_move_count);
        let base_production = self.robot_count.geode * moves_left;
        let staircase_production: usize = (1..=moves_left).sum();
        return self.resources.geode + base_production + staircase_production;
    }
}

const MAX_MOVE_COUNT: usize = 32;

struct GameStateValidator {
    states_by_move: HashMap<usize, Vec<GameState>>,
}

impl GameStateValidator {
    fn new() -> Self {
        Self {
            states_by_move: HashMap::new(),
        }
    }
    fn sequence_has_merit(&mut self, state: &GameState) -> bool {
        match self.states_by_move.get_mut(&state.move_number) {
            Some(vec) => {
                let better_state_exists = vec
                    .iter()
                    .any(|current_state| current_state.is_objectively_better_than(state));
                if better_state_exists {
                    return false;
                }
                // State does have merit, so filter out anything that it beats, and add it in
                let mut new_vec: Vec<GameState> = vec
                    .drain(..)
                    .filter(|known_good_state| !state.is_objectively_better_than(known_good_state))
                    .collect();
                new_vec.push(state.clone());
                self.states_by_move.insert(state.move_number, new_vec);
                true
            }
            None => {
                self.states_by_move
                    .insert(state.move_number, vec![state.clone()]);
                true
            }
        }
    }
}

struct Game {
    blueprint: Blueprint,
    states: Vec<GameState>,
    max_move_count: usize,
}

impl Game {
    fn new(blueprint: Blueprint, max_move_count: usize) -> Self {
        let game_state = GameState::new();
        return Self {
            blueprint,
            max_move_count,
            states: vec![game_state],
        };
    }
    fn get_quality_level(&mut self) -> usize {
        self.get_max_geodes() * self.blueprint.id
    }
    fn get_max_geodes(&mut self) -> usize {
        let mut highest_geode_count = 0;
        let mut considered_count = 0;
        let mut considered_states = HashSet::new();
        let mut skipped_for_considered_states_before_add = 0;
        let mut highest_geode_state = GameState::new();
        let mut skipped_for_max_geode = 0;

        let mut state_validator = GameStateValidator::new();
        let mut skipped_from_validator = 0;

        while let Some(state) = self.states.pop() {
            considered_count = considered_count + 1;
            if state.move_number == self.max_move_count {
                if state.resources.geode > highest_geode_count {
                    println!("Reset highest geode count to {:?}", state.resources.geode);
                    highest_geode_count = state.resources.geode;
                    highest_geode_state = state;
                }
                continue;
            }
            for next_move in state.generate_all_next_moves(&self.blueprint).into_iter() {
                if considered_states.contains(&next_move) {
                    skipped_for_considered_states_before_add =
                        skipped_for_considered_states_before_add + 1;
                    continue;
                } else if next_move.maximum_possible_geode_at_end(MAX_MOVE_COUNT)
                    < highest_geode_count
                {
                    skipped_for_max_geode = skipped_for_max_geode + 1;

                    //                } else if !state_validator.sequence_has_merit(&next_move) {
                    //                    skipped_from_validator = skipped_from_validator + 1;
                    //                    continue;
                } else {
                    considered_states.insert(next_move.clone());
                    self.states.push(next_move);
                }
            }
        }

        println!("Best state is {:#?}", highest_geode_state);
        println!("For blueprint {:#?}", self.blueprint);
        println!("We looked at  {:?}", considered_count);
        println!(
            "We skipped {:?} because a duplicate had been considered before this one",
            skipped_for_considered_states_before_add
        );
        println!("We skipped {:?} from the validator", skipped_from_validator);
        println!(
            "We skipped {:?} from the max lookahead",
            skipped_for_max_geode
        );
        println!("Most geodes are {:?}", highest_geode_count);
        return highest_geode_count;
    }
}

fn part_1(input: &str) -> usize {
    let blueprints: Vec<Blueprint> = input.lines().map(|line| line.parse().unwrap()).collect();
    let total_quality = blueprints
        .into_iter()
        .map(|blueprint| Game::new(blueprint, MAX_MOVE_COUNT))
        .map(|mut game| game.get_quality_level())
        .sum();

    return total_quality;
}

fn part_2(input: &str) -> usize {
    let blueprints: Vec<Blueprint> = input.lines().map(|line| line.parse().unwrap()).collect();
    let total_quality = blueprints[..]
        .into_iter()
        .map(|blueprint| Game::new(*blueprint, 32))
        .map(|mut game| game.get_max_geodes())
        .product();

    return total_quality;
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn production_lookahead() {
        let game_state = GameState::new();

        let future_production = game_state.maximum_possible_geode_at_end(3);

        assert_eq!(future_production, 6);
    }

    #[test]
    fn production_lookahead_with_base() {
        let mut game_state = GameState::new();
        game_state.robot_count.geode = 2;

        let future_production = game_state.maximum_possible_geode_at_end(3);

        assert_eq!(future_production, 12);
    }

    #[test]
    fn production_lookahead_with_inital() {
        let mut game_state = GameState::new();
        game_state.robot_count.geode = 2;
        game_state.resources.geode = 2;

        let future_production = game_state.maximum_possible_geode_at_end(3);

        assert_eq!(future_production, 14);
    }

    //    #[test]
    //    fn part_1_given() {
    //        let input = include_str!("../input.dev");
    //
    //        let output = part_1(input);
    //
    //        assert_eq!(output, 33);
    //    }

    //    #[test]
    //    fn basic_move_generation() {
    //        let blueprint = generate_1_cost_blueprint();
    //        let game_state = GameState::new();
    //
    //        let next_game_states = game_state.generate_all_next_moves(&blueprint);
    //
    //        assert_eq!(next_game_states, 33);
    //    }

    #[test]
    fn parse_blueprint() {
        let blueprint_str = "Blueprint 30: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 4 ore and 20 clay. Each geode robot costs 2 ore and 15 obsidian.";
        let expected_blueprint = Blueprint {
            id: 30,
            ore_robot_cost: Resources {
                ore: 4,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            clay_robot_cost: Resources {
                ore: 3,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            obsidian_robot_cost: Resources {
                ore: 4,
                clay: 20,
                obsidian: 0,
                geode: 0,
            },
            geode_robot_cost: Resources {
                ore: 2,
                clay: 0,
                obsidian: 15,
                geode: 0,
            },
        };

        let blueprint: Blueprint = blueprint_str.parse().unwrap();

        assert_eq!(expected_blueprint, blueprint);
    }
    fn generate_1_cost_blueprint() -> Blueprint {
        return Blueprint {
            id: 1,
            ore_robot_cost: Resources {
                ore: 1,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            clay_robot_cost: Resources {
                ore: 1,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            obsidian_robot_cost: Resources {
                ore: 1,
                clay: 1,
                obsidian: 0,
                geode: 0,
            },
            geode_robot_cost: Resources {
                ore: 1,
                clay: 0,
                obsidian: 1,
                geode: 0,
            },
        };
    }
}
