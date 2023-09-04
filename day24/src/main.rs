use std::cmp::Ordering;
use std::collections::HashSet;
use std::time::Instant;
use std::{collections::BinaryHeap, str::FromStr};

fn main() {
    println!("Hello, world!");
    let start = Instant::now();
    let test_input = include_str!("../input.prod");
    let result = solve(test_input, Version::Part2);

    println!("Result is {result}, in {:?}", start.elapsed());
}

enum Version {
    Part1,
    Part2,
}

fn solve(input: &str, version: Version) -> usize {
    let board: Board = input.parse().unwrap();
    let mut future_game_states = BinaryHeap::new();
    let start_move = GameState {
        part_of_game: PartOfGame::HeadingToEnd,
        move_number: 0,
        position: Coordinate { x: 1, y: 0 },
    };
    let mut considered_states: HashSet<GameState> = HashSet::new();
    future_game_states.push(Priority(start_move.score(&board), start_move));
    let mut fewest_moves = usize::MAX;
    while let Some(Priority(_, state)) = future_game_states.pop() {
        if state.is_complete(&board, &version) {
            if state.move_number < fewest_moves {
                println!("Found a shorter finishing move of {:?}", state.move_number);
                fewest_moves = state.move_number;
            }
            continue;
        }
        //println!("Considering state: {state:?}");
        if state.move_number + board.distance_to_end(&state.position) > fewest_moves {
            //            println!("Could not make it end in time");
            continue;
        }

        let valid_moves = state.next_moves(&board, &version);

        for valid_move in valid_moves.into_iter() {
            if considered_states.get(&valid_move) == None {
                considered_states.insert(valid_move.clone());
                future_game_states.push(Priority(valid_move.score(&board), valid_move));
            }
        }
    }
    fewest_moves
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum PartOfGame {
    HeadingToEnd,
    HeadingBackToStart,
    HeadingBackToEnd,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct GameState {
    part_of_game: PartOfGame,
    move_number: usize,
    position: Coordinate,
}

impl GameState {
    fn score(&self, board: &Board) -> usize {
        (match self.part_of_game {
            PartOfGame::HeadingToEnd => self.position.x + self.position.y,
            PartOfGame::HeadingBackToStart => {
                100_000 + board.width - self.position.x + board.height - self.position.y
            }
            PartOfGame::HeadingBackToEnd => 100_000_000 + self.position.x + self.position.y,
        }) as usize
    }
    fn is_complete(&self, board: &Board, version: &Version) -> bool {
        let space = board.at(&self.position, self.move_number);
        match *version {
            Version::Part1 => self.part_of_game == PartOfGame::HeadingToEnd && space == Space::End,
            Version::Part2 => {
                self.part_of_game == PartOfGame::HeadingBackToEnd && space == Space::End
            }
        }
    }
    fn next_moves(self, board: &Board, version: &Version) -> Vec<GameState> {
        let next_move_number = self.move_number + 1;
        let space = board.at(&self.position, self.move_number);
        let part_of_game = match *version {
            Version::Part1 => self.part_of_game,
            Version::Part2 => {
                if space == Space::End && self.part_of_game == PartOfGame::HeadingToEnd {
                    PartOfGame::HeadingBackToStart
                } else if space == Space::Start
                    && self.part_of_game == PartOfGame::HeadingBackToStart
                {
                    PartOfGame::HeadingBackToEnd
                } else {
                    self.part_of_game
                }
            }
        };

        let new_states = vec![
            GameState {
                part_of_game,
                move_number: next_move_number,
                position: self.position.clone(),
            },
            GameState {
                part_of_game,
                move_number: next_move_number,
                position: self.position.left(1),
            },
            GameState {
                part_of_game,
                move_number: next_move_number,
                position: self.position.up(1),
            },
            GameState {
                part_of_game,
                move_number: next_move_number,
                position: self.position.right(1),
            },
            GameState {
                part_of_game,
                move_number: next_move_number,
                position: self.position.down(1),
            },
        ];
        let valid_moves: Vec<GameState> = new_states
            .into_iter()
            .filter(|state| {
                let space = board.at(&state.position, state.move_number);
                space == Space::Empty || space == Space::End || space == Space::Start
            })
            .collect();

        valid_moves
    }
}

#[derive(Clone, Debug, PartialEq, Hash)]
pub enum Direction {
    Up,
    Left,
    Down,
    Right,
    Unknown,
}

#[derive(Clone, Debug, PartialEq, Hash)]
pub enum Space {
    Empty,
    Wall,
    End,
    Start,
    Blizzard(Direction),
}

impl From<char> for Space {
    fn from(c: char) -> Self {
        match c {
            '#' => Self::Wall,
            '.' => Self::Empty,
            '>' => Self::Blizzard(Direction::Right),
            '^' => Self::Blizzard(Direction::Up),
            'v' => Self::Blizzard(Direction::Down),
            '<' => Self::Blizzard(Direction::Left),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn up(&self, spaces: usize) -> Self {
        Coordinate {
            x: self.x,
            y: self.y - (spaces as i32),
        }
    }
    fn down(&self, spaces: usize) -> Self {
        Coordinate {
            x: self.x,
            y: self.y + (spaces as i32),
        }
    }
    fn left(&self, spaces: usize) -> Self {
        Coordinate {
            x: self.x - (spaces as i32),
            y: self.y,
        }
    }
    fn right(&self, spaces: usize) -> Self {
        Coordinate {
            x: self.x + (spaces as i32),
            y: self.y,
        }
    }
    fn new(x: i32, y: i32) -> Self {
        Coordinate { x, y }
    }
}

#[derive(PartialEq, Debug)]
pub struct Board {
    board: Vec<Vec<Space>>,
    width: i32,
    height: i32,
}

impl Board {
    fn distance_to_end(&self, coordinate: &Coordinate) -> usize {
        ((self.width - coordinate.x) + (self.height - coordinate.y)) as usize
    }
    fn at_initial_position(&self, coordinate: &Coordinate) -> Space {
        let x = coordinate.x.rem_euclid(self.width);
        let y = coordinate.y.rem_euclid(self.height);
        self.board[y as usize][x as usize].clone()
    }
    fn at_initial_position_blizzard(&self, coordinate: &Coordinate) -> Space {
        let x = (coordinate.x - 1).rem_euclid(self.width - 2) + 1;
        let y = (coordinate.y - 1).rem_euclid(self.height - 2) + 1;
        self.board[y as usize][x as usize].clone()
    }
    pub fn at(&self, coordinate: &Coordinate, move_number: usize) -> Space {
        if !(coordinate.x.rem_euclid(self.width) == 0
            || coordinate.x.rem_euclid(self.width) == self.width - 1
            || coordinate.y.rem_euclid(self.height) == 0
            || coordinate.y.rem_euclid(self.height) == self.height - 1)
        {
            let up_space = self.at_initial_position_blizzard(&coordinate.up(move_number));
            if Space::Blizzard(Direction::Down) == up_space {
                return Space::Blizzard(Direction::Unknown);
            }
            let down_space = self.at_initial_position_blizzard(&coordinate.down(move_number));
            if Space::Blizzard(Direction::Up) == down_space {
                return Space::Blizzard(Direction::Unknown);
            }
            let left_space = self.at_initial_position_blizzard(&coordinate.left(move_number));
            if Space::Blizzard(Direction::Right) == left_space {
                return Space::Blizzard(Direction::Unknown);
            }
            let right_space = self.at_initial_position_blizzard(&coordinate.right(move_number));
            if Space::Blizzard(Direction::Left) == right_space {
                return Space::Blizzard(Direction::Unknown);
            }
        }
        let initial_space = self.at_initial_position(&coordinate);
        match initial_space {
            Space::Blizzard(_) => Space::Empty,
            other => other,
        }
    }
}

impl FromStr for Board {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut board: Vec<Vec<Space>> = s
            .lines()
            .map(|line| line.trim().chars().map(|c| c.into()).collect())
            .collect();
        let width: i32 = board[0].len().try_into().unwrap();
        let height: i32 = board.len().try_into().unwrap();
        board[0][1] = Space::Start;
        board[(height - 1) as usize][(width - 2) as usize] = Space::End;
        Ok(Self {
            width,
            height,
            board,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn part_1_given() {
        let input = include_str!("../input.dev");

        let result = solve(input, Version::Part1);

        assert_eq!(result, 18);
    }

    #[test]
    fn part_2_given() {
        let input = include_str!("../input.dev");

        let result = solve(input, Version::Part2);

        assert_eq!(result, 54);
    }

    #[test]
    fn parse_small_board() {
        let small_board = "#.##\n#<>#\n#^v#\n#..#\n##.#";
        let expected_board = Board {
            board: vec![
                vec![Space::Wall, Space::Start, Space::Wall, Space::Wall],
                vec![
                    Space::Wall,
                    Space::Blizzard(Direction::Left),
                    Space::Blizzard(Direction::Right),
                    Space::Wall,
                ],
                vec![
                    Space::Wall,
                    Space::Blizzard(Direction::Up),
                    Space::Blizzard(Direction::Down),
                    Space::Wall,
                ],
                vec![Space::Wall, Space::Empty, Space::Empty, Space::Wall],
                vec![Space::Wall, Space::Wall, Space::End, Space::Wall],
            ],
            width: 4,
            height: 5,
        };

        let board: Board = small_board.parse().unwrap();

        assert_eq!(board, expected_board);
    }

    #[test]
    fn find_wall_left() {
        let small_board = "#.##\n#<>#\n#^v#\n#..#\n##.#";
        let board: Board = small_board.parse().unwrap();

        let wall_left = board.at(&Coordinate::new(0, 1), 0);

        assert_eq!(wall_left, Space::Wall);
    }

    #[test]
    fn find_wall_right() {
        let small_board = "#.##\n#<>#\n#^v#\n#..#\n##.#";
        let board: Board = small_board.parse().unwrap();

        let wall_left = board.at(&Coordinate::new(3, 1), 0);

        assert_eq!(wall_left, Space::Wall);
    }

    #[test]
    fn find_blizzard_move_0() {
        let small_board = "#.##\n#<>#\n#^v#\n#..#\n##.#";
        let board: Board = small_board.parse().unwrap();

        let wall_left = board.at(&Coordinate::new(1, 1), 0);

        assert_eq!(wall_left, Space::Blizzard(Direction::Unknown));
    }

    #[test]
    fn blizzard_moves_out_of_space() {
        let small_board = "#.##\n#<>#\n#^v#\n#..#\n##.#";
        let board: Board = small_board.parse().unwrap();

        let wall_left = board.at(&Coordinate::new(2, 2), 1);

        assert_eq!(wall_left, Space::Empty);
    }

    #[test]
    fn blizzard_moves_into_down() {
        let small_board = "#.##\n#<>#\n#^v#\n#..#\n##.#";
        let board: Board = small_board.parse().unwrap();

        let wall_left = board.at(&Coordinate::new(2, 3), 1);

        assert_eq!(wall_left, Space::Blizzard(Direction::Unknown));
    }

    #[test]
    fn blizzard_moves_into_up() {
        let small_board = "#.##\n#<>#\n#^v#\n#..#\n##.#";
        let board: Board = small_board.parse().unwrap();

        let wall_left = board.at(&Coordinate::new(1, 3), 2);

        assert_eq!(wall_left, Space::Blizzard(Direction::Unknown));
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
