use anyhow::anyhow;
use std::{fs, str::FromStr};

fn main() {
    println!("Hello, world!");
    let input = include_str!("./input.txt");
    let count = part_1(&input);
    println!("Part 1: {:?}", count);
}

struct RangePair {
    first: Range,
    second: Range,
}

impl RangePair {
    fn fully_overlaps(&self) -> bool {
        self.first.overlaps(&self.second) || self.second.overlaps(&self.first)
    }
    fn fully_overlaps_score(&self) -> usize {
        match self.fully_overlaps() {
            true => 1,
            false => 0,
        }
    }
    fn partially_overlaps(&self) -> bool {
        self.first.partially_overlaps(&self.second) || self.second.partially_overlaps(&self.first)
    }
}

impl FromStr for RangePair {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (o, p) = match s.split_once(",") {
            Some((o, p)) => (o, p),
            None => return Err(anyhow::anyhow!("invalid input")),
        };
        let first = str::parse::<Range>(o)?;
        let second = str::parse::<Range>(p)?;
        Ok(Self { first, second })
    }
}

struct Range {
    left: i32,
    right: i32,
}

impl Range {
    fn overlaps(&self, other: &Self) -> bool {
        return (self.right >= other.right && self.left <= other.left)
            || (other.right >= self.right && other.left <= self.left);
    }
}

impl FromStr for Range {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = match s.split_once('-') {
            Some((l, r)) => (l, r),
            None => return Err(anyhow::anyhow!("invalid input")),
        };
        let left = match str::parse::<i32>(left) {
            Ok(value) => value,
            Err(_) => return Err(anyhow::anyhow!("invalid input")),
        };
        let right = match str::parse::<i32>(right) {
            Ok(value) => value,
            Err(_) => return Err(anyhow::anyhow!("invalid input")),
        };
        Ok(Self { left, right })
    }
}

pub fn part_1(input: &'static str) -> usize {
    let count = input
        .lines()
        .flat_map(|line| line.parse::<RangePair>())
        .filter(|pair| pair.fully_overlaps())
        .count();
    return count;
}

pub fn part_1_map(input: &'static str) -> usize {
    let count = parse_to_range_pairs(input)
        .map(|pair| pair.fully_overlaps_score())
        .sum();
    return count;
}

fn part_2(input: &'static str) -> usize {
    let count = parse_to_range_pairs(input)
        .filter(|pair| pair.partially_overlaps())
        .count();
    return count;
}

fn parse_to_range_pairs(input: &'static str) -> impl Iterator<Item = RangePair> {
    input.lines().flat_map(|line| line.parse::<RangePair>())
}

#[cfg(test)]
mod test {
    use crate::*;
    use std::time::Instant;

    #[test]
    fn aoc_solution_1() {
        let input = include_str!("./input.txt");

        let count = part_1(&input);

        assert_eq!(count, 441);
    }

    #[test]
    fn aoc_solution_filter() {
        let input = include_str!("./input.txt");
        let now = Instant::now();

        let count = part_1(&input);
        let elapsed = now.elapsed();
        println!("Elapsed (filter) : {:.2?}", elapsed);

        assert_eq!(count, 441);
    }

    #[test]
    fn aoc_solution_map() {
        let input = include_str!("./input.txt");
        let now = Instant::now();

        let count = part_1_map(&input);
        let elapsed = now.elapsed();
        println!("Elapsed (map) : {:.2?}", elapsed);

        assert_eq!(count, 441);
    }

    #[test]
    fn aoc_solution_2() {
        let input = include_str!("./input.txt");

        let count = part_2(&input);

        assert_eq!(count, 861);
    }

    #[test]
    fn ignores_invalid_range_1() {
        let invalid_input = "1-10,3-9\n1-20,2.2-13";

        let count = part_1(invalid_input);

        assert_eq!(count, 1);
    }
}
