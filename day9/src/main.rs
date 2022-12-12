use std::collections::{HashSet, VecDeque};
use std::str::FromStr;
use std::time::Instant;

use anyhow::{Error, Result};

fn main() {
    println!("Hello, world!");
    let start = Instant::now();
    let result = part_2(include_str!("./full")).unwrap();
    let diff = Instant::now() - start;
    println!("Part 2 result: {result}, in {:?}", diff);
}

#[derive(Debug)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "R" => Direction::Right,
            "U" => Direction::Up,
            "L" => Direction::Left,
            "D" => Direction::Down,
            _ => unreachable!(),
        })
    }
}

#[derive(Debug)]
struct DirectionMove {
    direction: Direction,
    amount: i32,
}

#[derive(Debug)]
struct RelativeMove {
    from: Position,
    to: Position,
}

impl FromStr for DirectionMove {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (d, a) = s.split_once(" ").unwrap();
        let direction = d.parse().unwrap();
        let amount = a.parse().unwrap();

        return Ok(Self { direction, amount });
    }
}

#[derive(Clone, PartialEq, Hash, Eq, Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Knot {
    position: Position,
    history: Vec<Position>,
}

impl Position {
    fn minus(&self, other: &Position) -> Position {
        return Position {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}

impl Knot {
    fn new() -> Self {
        return Self {
            position: Position { x: 0, y: 0 },
            history: vec![Position { x: 0, y: 0 }],
        };
    }
    fn move_directly(&mut self, direct_move: DirectionMove) -> Vec<RelativeMove> {
        let move_history: Vec<RelativeMove> = (0..direct_move.amount)
            .map(|_| {
                let initial_position = self.position.clone();
                self.move_single_step(&direct_move.direction);
                let final_position = self.position.clone();
                return RelativeMove {
                    from: initial_position,
                    to: final_position,
                };
            })
            .collect();

        return move_history;
    }

    fn move_relatively(&mut self, relative_moves: &Vec<RelativeMove>) -> Vec<RelativeMove> {
        let propogated_moves = relative_moves
            .into_iter()
            .flat_map(|relative_move| {
                let initial_position = self.position.clone();
                let final_relative_position = relative_move.to.minus(&self.position);
                if final_relative_position.x.abs() < 2 && final_relative_position.y.abs() < 2 {
                    return None;
                } else if final_relative_position.x.abs() == 2
                    && final_relative_position.y.abs() == 2
                {
                    self.position.x = self.position.x + final_relative_position.x / 2;
                    self.position.y = self.position.y + final_relative_position.y / 2;
                } else if final_relative_position.x.abs() == 2 {
                    self.position.x = self.position.x + final_relative_position.x / 2;
                    self.position.y = relative_move.to.y;
                } else if final_relative_position.y.abs() == 2 {
                    self.position.y = self.position.y + final_relative_position.y / 2;
                    self.position.x = relative_move.to.x;
                } else {
                    unreachable!();
                }
                let move_done = RelativeMove {
                    from: initial_position,
                    to: self.position.clone(),
                };
                self.history.push(move_done.to.clone());

                return Some(move_done);
            })
            .collect();

        return propogated_moves;
    }

    fn move_single_step(&mut self, direction: &Direction) -> RelativeMove {
        let initial_position = self.position.clone();
        match direction {
            Direction::Up => self.position.y = self.position.y + 1,
            Direction::Down => self.position.y = self.position.y - 1,
            Direction::Left => self.position.x = self.position.x - 1,
            Direction::Right => self.position.x = self.position.x + 1,
        }
        let final_position = self.position.clone();
        return RelativeMove {
            from: initial_position,
            to: final_position,
        };
    }
}

struct Rope {
    head: Knot,
    following_knots: VecDeque<Knot>,
}

impl Rope {
    fn new(length: usize) -> Self {
        let mut following_knots = VecDeque::new();
        for _ in 0..length - 1 {
            following_knots.push_back(Knot::new())
        }
        Self {
            head: Knot::new(),
            following_knots,
        }
    }
}

impl Rope {
    fn tail(&self) -> &Knot {
        return self.following_knots.back().unwrap();
    }

    fn count_tail_positions(&self) -> usize {
        HashSet::<_>::from_iter(self.tail().history.iter()).len()
    }
}

//fn part_1(input: &str) -> Result<usize> {
//    let mut rope = Rope::new();
//    let moves: Vec<DirectionMove> = input.lines().map(|line| line.parse().unwrap()).collect();
//
//    for head_move in moves {
//        rope.move_head(head_move);
//    }
//
//    return Ok(rope.count_tail_positions());
//}

fn part_2(input: &str) -> Result<usize> {
    let mut rope = Rope::new(10);
    let moves: Vec<DirectionMove> = input.lines().map(|line| line.parse().unwrap()).collect();

    for head_move in moves {
        let mut relative_moves = rope.head.move_directly(head_move);
        for following_knot in rope.following_knots.iter_mut() {
            relative_moves = following_knot.move_relatively(&relative_moves);
        }
    }

    return Ok(rope.count_tail_positions());
}

#[cfg(test)]
mod test {
    use crate::*;

    //    #[test]
    //    fn given_test() {
    //        let input = include_str!("./given");
    //
    //        let result = part_1(input).unwrap();
    //
    //        assert_eq!(result, 13);
    //    }
    #[test]
    fn part_2_test() {
        let input = include_str!("./given_2");

        let result = part_2(input).unwrap();

        assert_eq!(result, 36);
    }
}
