use std::{collections::VecDeque, str::FromStr};

fn main() {
    println!("Hello, world!");
    let input = include_str!("../input.prod");
    let output = part_2(input);
    println!("output is {output}");
}

fn part_1(input: &str) -> usize {
    let mut grove: Grove = input.parse().unwrap();
    for i in 0..10 {
        let desired_moves = grove.get_all_requests();
        grove.process_all_requests(desired_moves);
        grove.cycle_direction_priorities();
        //        println!("Grove {i}: {:#?}", grove);
    }
    return grove.count_ground_squares();
}

fn part_2(input: &str) -> usize {
    let mut grove: Grove = input.parse().unwrap();
    let mut i = 0;
    loop {
        i = i + 1;
        println!("loop: {i}");
        let desired_moves = grove.get_all_requests();
        let moved_anything = grove.process_all_requests(desired_moves);
        if !moved_anything {
            break;
        }
        grove.cycle_direction_priorities();
        //        println!("Grove {i}: {:#?}", grove);
    }
    return i;
}

#[derive(Clone, Debug)]
enum Position {
    Empty,
    Elf,
}

impl From<char> for Position {
    fn from(c: char) -> Self {
        match c {
            '#' => Position::Elf,
            '.' => Position::Empty,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
struct Grove {
    squares: Vec<Vec<Position>>,
    direction_priorities: VecDeque<Direction>,
}

#[derive(Clone, Debug, PartialEq)]
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq)]
struct Move {
    from: Coordinate,
    to: Coordinate,
}

struct MoveConsideration<'a> {
    desired_positions: Vec<&'a Position>,
    destination: Coordinate,
}

#[derive(Debug, PartialEq)]
struct Edges {
    max_x: usize,
    min_x: usize,
    max_y: usize,
    min_y: usize,
}

impl Grove {
    fn new(size: usize) -> Self {
        Self {
            squares: vec![vec![Position::Empty; size]; size],
            direction_priorities: VecDeque::from(vec![
                Direction::North,
                Direction::South,
                Direction::West,
                Direction::East,
            ]),
        }
    }
    fn get_all_elves(&self) -> Vec<Coordinate> {
        self.squares
            .iter()
            .enumerate()
            .flat_map(|(i, vec)| {
                vec.iter().enumerate().filter_map(move |(j, position)| {
                    if let Position::Elf = position {
                        return Some(Coordinate { x: j, y: i });
                    } else {
                        return None;
                    }
                })
            })
            .collect()
    }
    fn count_ground_squares(&self) -> usize {
        let edges = self.find_edges();
        println!("Edges are: {:?}", edges);
        let ground_squares = self.squares[edges.min_y..=edges.max_y]
            .iter()
            .map(|row| {
                row[edges.min_x..=edges.max_x]
                    .iter()
                    .filter(|a| match a {
                        Position::Empty => true,
                        _ => false,
                    })
                    .count()
            })
            .sum();

        ground_squares
    }
    fn find_edges(&self) -> Edges {
        let min_x = self
            .squares
            .iter()
            .flat_map(|row| {
                row.iter().position(|position| match position {
                    Position::Elf => true,
                    _ => false,
                })
            })
            .min()
            .unwrap();
        let width = self.squares[0].len();
        let max_x = width
            - 1
            - self
                .squares
                .iter()
                .flat_map(|row| {
                    row.iter().rev().position(|position| match position {
                        Position::Elf => true,
                        _ => false,
                    })
                })
                .min()
                .unwrap();
        let min_y = self
            .squares
            .iter()
            .position(|row| {
                row.iter().any(|position| match position {
                    Position::Elf => true,
                    _ => false,
                })
            })
            .unwrap();
        let height = self.squares.len();
        let max_y = height
            - 1
            - self
                .squares
                .iter()
                .rev()
                .position(|row| {
                    row.iter().any(|position| match position {
                        Position::Elf => true,
                        _ => false,
                    })
                })
                .unwrap();

        return Edges {
            max_x,
            min_x,
            max_y,
            min_y,
        };
    }
    fn process_all_requests(&mut self, moves: Vec<Move>) -> bool {
        let mut moved_anything = false;
        for desired_move in moves.iter().filter(|m| m.to != m.from) {
            if !moves
                .iter()
                .any(|other_move| (other_move.to == desired_move.to) && other_move != desired_move)
            {
                moved_anything = true;
                self.squares[desired_move.from.y][desired_move.from.x] = Position::Empty;
                self.squares[desired_move.to.y][desired_move.to.x] = Position::Elf;
            }
        }
        return moved_anything;
    }
    fn get_all_requests(&self) -> Vec<Move> {
        let elves = self.get_all_elves();
        let requests: Vec<Move> = elves
            .into_iter()
            .map(|elf| {
                let mut mut_move = Move {
                    from: elf.clone(),
                    to: elf.clone(),
                };

                if self.elf_has_empty_surroundings(&elf) {
                    return mut_move;
                }

                for direction in self.direction_priorities.iter() {
                    let optional_move = self.elf_consider_direction(&elf, direction);
                    if let Some(concrete_move) = optional_move {
                        mut_move = Move {
                            from: elf,
                            to: concrete_move,
                        };
                        break;
                    }
                }

                return mut_move;
            })
            .collect();

        return requests;
    }

    fn cycle_direction_priorities(&mut self) {
        let front = self.direction_priorities.pop_front().unwrap();
        self.direction_priorities.push_back(front);
    }

    fn elf_has_empty_surroundings(&self, elf_position: &Coordinate) -> bool {
        let surroundings = vec![
            &self.squares[elf_position.y - 1][elf_position.x - 1],
            &self.squares[elf_position.y - 1][elf_position.x],
            &self.squares[elf_position.y - 1][elf_position.x + 1],
            &self.squares[elf_position.y][elf_position.x - 1],
            &self.squares[elf_position.y][elf_position.x + 1],
            &self.squares[elf_position.y + 1][elf_position.x - 1],
            &self.squares[elf_position.y + 1][elf_position.x],
            &self.squares[elf_position.y + 1][elf_position.x + 1],
        ];
        surroundings.iter().all(|position| match position {
            Position::Elf => false,
            Position::Empty => true,
        })
    }
    fn elf_consider_direction(
        &self,
        elf_position: &Coordinate,
        direction: &Direction,
    ) -> Option<Coordinate> {
        let desired_positions: MoveConsideration = match direction {
            Direction::North => MoveConsideration {
                desired_positions: self.squares[elf_position.y - 1]
                    [elf_position.x - 1..=elf_position.x + 1]
                    .iter()
                    .collect(),
                destination: Coordinate {
                    y: elf_position.y - 1,
                    x: elf_position.x,
                },
            },
            Direction::South => MoveConsideration {
                desired_positions: self.squares[elf_position.y + 1]
                    [elf_position.x - 1..=elf_position.x + 1]
                    .iter()
                    .collect(),

                destination: Coordinate {
                    y: elf_position.y + 1,
                    x: elf_position.x,
                },
            },
            Direction::East => MoveConsideration {
                desired_positions: vec![
                    &self.squares[elf_position.y - 1][elf_position.x + 1],
                    &self.squares[elf_position.y][elf_position.x + 1],
                    &self.squares[elf_position.y + 1][elf_position.x + 1],
                ],
                destination: Coordinate {
                    y: elf_position.y,
                    x: elf_position.x + 1,
                },
            },

            Direction::West => MoveConsideration {
                desired_positions: vec![
                    &self.squares[elf_position.y - 1][elf_position.x - 1],
                    &self.squares[elf_position.y][elf_position.x - 1],
                    &self.squares[elf_position.y + 1][elf_position.x - 1],
                ],
                destination: Coordinate {
                    y: elf_position.y,
                    x: elf_position.x - 1,
                },
            },
        };
        if desired_positions
            .desired_positions
            .iter()
            .any(|position| match position {
                Position::Empty => false,
                Position::Elf => true,
            })
        {
            return None;
        } else {
            return Some(desired_positions.destination);
        }
    }
}

impl FromStr for Grove {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grove = Grove::new(0);
        let unpadded_positions: Vec<Vec<Position>> = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|position_char| position_char.into())
                    .collect()
            })
            .collect();

        let width = unpadded_positions[0].len();
        let height = unpadded_positions.len();
        let padding = 150;
        let mut padded_grove =
            vec![vec![Position::Empty; width + 2 * padding]; height + 2 * padding];
        unpadded_positions
            .into_iter()
            .enumerate()
            .for_each(|(i, vec)| {
                vec.into_iter()
                    .enumerate()
                    .for_each(|(j, position)| padded_grove[i + padding][j + padding] = position)
            });

        grove.squares = padded_grove;

        Ok(grove)
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn part_1_given() {
        let input = include_str!("../input.dev");

        let output = part_1(input);

        assert_eq!(output, 110);
    }

    #[test]
    fn consider_north() {
        let mut grove = Grove::new(5);
        grove.squares[2][2] = Position::Elf;
        let expected_coordinate = Coordinate { x: 2, y: 1 };

        let desired_cordinate = grove
            .elf_consider_direction(&Coordinate { x: 2, y: 2 }, &Direction::North)
            .unwrap();

        assert_eq!(desired_cordinate, expected_coordinate);
    }

    #[test]
    fn consider_north_blocked() {
        let mut grove = Grove::new(5);
        grove.squares[2][2] = Position::Elf;
        grove.squares[1][2] = Position::Elf;

        let desired_coordinate =
            grove.elf_consider_direction(&Coordinate { x: 2, y: 2 }, &Direction::North);

        assert_eq!(desired_coordinate, None);
    }

    #[test]
    fn consider_north_east_blocked() {
        let mut grove = Grove::new(5);
        grove.squares[2][2] = Position::Elf;
        grove.squares[1][3] = Position::Elf;

        let desired_coordinate =
            grove.elf_consider_direction(&Coordinate { x: 2, y: 2 }, &Direction::North);

        assert_eq!(desired_coordinate, None);
    }

    #[test]
    fn consider_east() {
        let mut grove = Grove::new(5);
        grove.squares[2][2] = Position::Elf;
        let expected_coordinate = Coordinate { x: 3, y: 2 };

        let desired_cordinate = grove
            .elf_consider_direction(&Coordinate { x: 2, y: 2 }, &Direction::East)
            .unwrap();

        assert_eq!(desired_cordinate, expected_coordinate);
    }

    #[test]
    fn consider_east_blocked() {
        let mut grove = Grove::new(5);
        grove.squares[2][2] = Position::Elf;
        grove.squares[1][3] = Position::Elf;

        let desired_coordinate =
            grove.elf_consider_direction(&Coordinate { x: 2, y: 2 }, &Direction::East);

        println!("Grove: {:?}", grove);

        assert_eq!(desired_coordinate, None);
    }

    #[test]
    fn get_all_elves() {
        let mut grove = Grove::new(5);
        grove.squares[2][2] = Position::Elf;
        grove.squares[1][2] = Position::Elf;
        let expected_elves = vec![Coordinate { x: 2, y: 1 }, Coordinate { x: 2, y: 2 }];

        let elves = grove.get_all_elves();

        assert_eq!(expected_elves, elves);
    }

    #[test]
    fn basic_empty_surroundings() {
        let mut grove = Grove::new(5);
        grove.squares[2][2] = Position::Elf;

        let surroundings_empty = grove.elf_has_empty_surroundings(&Coordinate { x: 2, y: 2 });

        assert_eq!(surroundings_empty, true);
    }

    #[test]
    fn basic_not_empty_surroundings() {
        let mut grove = Grove::new(5);
        grove.squares[2][2] = Position::Elf;
        grove.squares[1][2] = Position::Elf;

        let surroundings_empty = grove.elf_has_empty_surroundings(&Coordinate { x: 2, y: 2 });

        assert_eq!(surroundings_empty, false);
    }

    #[test]
    fn find_edges() {
        let mut grove = Grove::new(5);
        grove.squares[2][3] = Position::Elf;
        grove.squares[1][2] = Position::Elf;
        let expected_edges = Edges {
            min_x: 2,
            max_x: 3,
            min_y: 1,
            max_y: 2,
        };

        let edges = grove.find_edges();

        assert_eq!(edges, expected_edges);
    }

    #[test]
    fn count_ground() {
        let mut grove = Grove::new(5);
        grove.squares[3][4] = Position::Elf;
        grove.squares[1][2] = Position::Elf;

        let ground_squares = grove.count_ground_squares();

        assert_eq!(ground_squares, 7);
    }
}
