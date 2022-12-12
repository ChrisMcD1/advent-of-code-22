use std::{collections::VecDeque, str::FromStr};

use anyhow::Result;

fn main() {
    println!("Hello, world!");
    let output = part_2(include_str!("./part_1_real")).unwrap();
    println!("{output}");
}

fn part_1(input: &str) -> Result<String> {
    let mut block_set = parse_blocks(input).unwrap();
    let (_, moves_str) = input.split_once("\n\n").unwrap();
    let moves: Vec<BlockMove> = moves_str.lines().flat_map(|line| line.parse()).collect();
    for block_move in moves {
        for _ in 0..block_move.quantity {
            let temp = block_set[block_move.source - 1].pop_back().unwrap();
            block_set[block_move.destination - 1].push_back(temp);
        }
    }
    let output: String = block_set
        .into_iter()
        .map(|mut column| column.pop_back().unwrap())
        .collect();

    return Ok(output);
}

fn part_2(input: &str) -> Result<String> {
    let mut block_set = parse_blocks(input).unwrap();
    let (_, moves_str) = input.split_once("\n\n").unwrap();
    let moves: Vec<BlockMove> = moves_str.lines().flat_map(|line| line.parse()).collect();
    for block_move in moves {
        let mut temp_vec = VecDeque::new();
        for _ in 0..block_move.quantity {
            temp_vec.push_front(block_set[block_move.source - 1].pop_back().unwrap());
        }
        temp_vec.into_iter().for_each(|elem| {
            block_set[block_move.destination - 1].push_back(elem);
        });
    }
    let output: String = block_set
        .into_iter()
        .map(|mut column| column.pop_back().unwrap())
        .collect();

    return Ok(output);
}

fn parse_blocks(input: &str) -> Result<Vec<VecDeque<char>>> {
    let (blocks, rest) = input.split_once("1").expect("Have a 1 you man");
    let col_count = rest
        .split_once("\n")
        .unwrap()
        .0
        .split(" ")
        .map(|num_str| num_str.parse::<usize>().unwrap_or(0))
        .max()
        .unwrap();
    println!("Col count {col_count}");
    let mut col_base: Vec<VecDeque<char>> = (1..=col_count).map(|_| VecDeque::new()).collect();
    println!("Col count {:?}", col_base);
    blocks
        .lines()
        .filter(|line| line.len() > 1)
        .for_each(|line| {
            for col in 1..=col_count {
                let found_char = line.as_bytes()[4 * (col - 1) + 1] as char;
                if found_char != ' ' {
                    col_base[col - 1].push_front(found_char)
                }
            }
        });

    return Ok(col_base);
}

#[derive(Debug, PartialEq)]
struct BlockMove {
    source: usize,
    destination: usize,
    quantity: usize,
}

impl FromStr for BlockMove {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, rest) = s.split_once("move").unwrap();
        let (quantity, rest) = rest.split_once("from").unwrap();
        let (source, destination) = rest.split_once("to").unwrap();
        Ok(Self {
            source: source.trim().parse().unwrap(),
            destination: destination.trim().parse().unwrap(),
            quantity: quantity.trim().parse().unwrap(),
        })
    }
}

#[cfg(test)]
mod test {
    use std::collections::VecDeque;

    use crate::{parse_blocks, part_1, part_2, BlockMove};

    #[test]
    fn part_1_given() {
        let input = include_str!("./part_1_given");

        let sorted = part_1(input);

        assert_eq!(sorted.unwrap(), "CMZ")
    }

    #[test]
    fn part_2_given() {
        let input = include_str!("./part_1_given");

        let sorted = part_2(input);

        assert_eq!(sorted.unwrap(), "MCD")
    }

    #[test]
    fn parse_blocks_given() {
        let input = include_str!("./part_1_given");
        let expected_output = vec![
            VecDeque::from(vec!['Z', 'N']),
            VecDeque::from(vec!['M', 'C', 'D']),
            VecDeque::from(vec!['P']),
        ];

        let sorted = parse_blocks(input);

        assert_eq!(sorted.unwrap(), expected_output)
    }

    #[test]
    fn parse_block_move() {
        let input = "move 1 from 2 to 3";
        let expected_output = BlockMove {
            source: 2,
            destination: 3,
            quantity: 1,
        };

        let parsed: BlockMove = input.parse().unwrap();

        assert_eq!(parsed, expected_output);
    }
}
