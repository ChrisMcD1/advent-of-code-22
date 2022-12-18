use std::str::FromStr;

use anyhow::{anyhow, Result};

fn main() {
    println!("Hello, world!");
    let input = include_str!("test_1");
    //let output = part_1(input).unwrap();
    let output: Vec<char> = part_2(input).unwrap();
    let output_chunks = output.chunks(40);
    for chunk in output_chunks {
        println!("{:?}", chunk.iter().collect::<String>());
    }
}

enum Command {
    Noop,
    Addx(i32),
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        //       let split_result = s.split_once(" ").unwrap_or((commmand, ""));
        //       if split_result.is_none() {
        //           return Err(anyhow!("Bad, in from_str for value {:?}", s));
        //       }
        let (command_str, value) = s.split_once(" ").unwrap_or(("noop", ""));
        //        let (command_str, value) = s.split_once(" ").unwrap();
        let command = match command_str {
            "addx" => Command::Addx(value.parse()?),
            "noop" => Command::Noop,
            _ => unreachable!(),
        };
        return Ok(command);
    }
}

fn part_1(input: &str) -> Result<i32> {
    let mut x = 1;
    let mut cycle_count = 0;

    let mut option_to_values = Vec::new();

    for line in input.lines() {
        let command: Command = line.parse()?;
        match command {
            Command::Noop => {
                cycle_count = cycle_count + 1;
                option_to_values.push(get_v_if_necessary(&x, &cycle_count));
            }
            Command::Addx(v) => {
                cycle_count = cycle_count + 1;
                option_to_values.push(get_v_if_necessary(&x, &cycle_count));
                cycle_count = cycle_count + 1;
                option_to_values.push(get_v_if_necessary(&x, &cycle_count));
                x = x + v;
            }
        }
    }
    let values = option_to_values
        .into_iter()
        .flat_map(|v| v)
        .filter(|val| [20, 60, 100, 140, 180, 220].contains(&val.1))
        .map(|val| val.0)
        .collect::<Vec<i32>>();

    println!("{:?}", values);

    let signal_strength: i32 = values.into_iter().sum();
    return Ok(signal_strength);
}

fn part_2(input: &str) -> Result<Vec<char>> {
    let mut screen: Vec<char> = vec!['.'; 242];
    let mut x = 1;
    let mut cycle_count = 0;

    for line in input.lines() {
        let command: Command = line.parse()?;
        match command {
            Command::Noop => {
                screen[cycle_count] = sprite_overlaps_cycle(&x, &(cycle_count as i32));
                cycle_count = cycle_count + 1;
            }
            Command::Addx(v) => {
                screen[cycle_count] = sprite_overlaps_cycle(&x, &(cycle_count as i32));
                cycle_count = cycle_count + 1;
                screen[cycle_count] = sprite_overlaps_cycle(&x, &(cycle_count as i32));
                cycle_count = cycle_count + 1;
                x = x + v;
            }
        }
    }

    return Ok(screen);
}

fn sprite_overlaps_cycle(x: &i32, cycle_count: &i32) -> char {
    if (cycle_count % 40 - x).abs() <= 1 {
        return '#';
    } else {
        return '.';
    }
}

fn get_v_if_necessary(x: &i32, cycle_count: &i32) -> Option<(i32, i32)> {
    if cycle_count % 20 == 0 {
        return Some((*x * *cycle_count, *cycle_count));
    } else {
        return None;
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn part_1_given() {
        let input = include_str!("given_1");

        let result = part_1(input).unwrap();

        assert_eq!(result, 13140);
    }

    #[test]
    fn part_2_given() {
        let input = include_str!("given_1");
        let expected_result: Vec<char> = "##..##..##..##..##..##..##..##..##..##..###...###...###...###...###...###...###.####....####....####....####....####....#####.....#####.....#####.....#####.....######......######......######......###########.......#######.......#######.....".chars().collect();

        let result = part_2(input).unwrap();

        assert_eq!(result, expected_result);
    }
}
