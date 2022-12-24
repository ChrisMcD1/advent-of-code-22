use anyhow::Result;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    println!("Hello, world!");
}

struct Valve {
    key: String,
    flow_rate: usize,
    tunnels: Vec<String>,
}

impl FromStr for Valve {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (flow, tunnels_section) = s.split_once(";").unwrap();
        let key_regex: Regex = Regex::new(r"Valve (\S*) has").unwrap();
        let captures = key_regex.captures(flow).unwrap();
        println!("Capture is {:?}", captures);
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
        let tunnels: Vec<String> = tunnels
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

fn part_1(input: &str) -> Result<usize> {
    let valves: Vec<Valve> = input.lines().map(|line| line.parse().unwrap()).collect();

    return Ok(0);
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn part_1_given() {
        let input = include_str!("../data.test");

        let result = part_1(input).unwrap();

        assert_eq!(result, 1651);
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
        let expected_tunnels = vec![String::from("DD"), String::from("II"), String::from("BB")];

        let valve: Valve = input.parse().unwrap();
        let tunnels = valve.tunnels;

        assert_eq!(tunnels, expected_tunnels);
    }

    #[test]
    fn parse_tunnel() {
        let input = "Valve ABC has flow rate=100; tunnels lead to valve DD";
        let expected_tunnels = vec![String::from("DD")];

        let valve: Valve = input.parse().unwrap();
        let tunnels = valve.tunnels;

        assert_eq!(tunnels, expected_tunnels);
    }
}
