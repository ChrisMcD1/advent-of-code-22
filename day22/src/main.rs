use regex::Regex;
use std::cmp::min;
use std::str::FromStr;
use std::time::Instant;

fn main() {
    println!("Hello, world!");
    let input = include_str!("../input.real");
    let start = Instant::now();
    let output = part_1(input);
    println!("We took {:?} time", Instant::now() - start);
    println!("Output is {:?}", output);
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Tile {
    Wall,
    Open,
    Nothing,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Orientation {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone, PartialEq, Debug)]
enum Move {
    Amount(usize),
    Turn(Rotation),
}

#[derive(Clone, PartialEq, Debug)]
enum Rotation {
    Clockwise,
    CounterClockwise,
}

#[derive(Clone, PartialEq, Debug)]
struct Position {
    x: usize,
    y: usize,
    orientation: Orientation,
}

#[derive(PartialEq, Debug)]
struct Game {
    board_height: usize,
    board_width: usize,
    board: Vec<Vec<Tile>>,
    moves: Vec<Move>,
    position: Position,
}

impl Game {
    fn turn(&mut self, rotation: &Rotation) {
        match rotation {
            Rotation::Clockwise => match self.position.orientation {
                Orientation::Right => self.position.orientation = Orientation::Down,
                Orientation::Left => self.position.orientation = Orientation::Up,
                Orientation::Up => self.position.orientation = Orientation::Right,
                Orientation::Down => self.position.orientation = Orientation::Left,
            },
            Rotation::CounterClockwise => match self.position.orientation {
                Orientation::Right => self.position.orientation = Orientation::Up,
                Orientation::Left => self.position.orientation = Orientation::Down,
                Orientation::Up => self.position.orientation = Orientation::Left,
                Orientation::Down => self.position.orientation = Orientation::Right,
            },
        }
    }
    fn get_next_position(&self) -> Position {
        match self.position.orientation {
            Orientation::Right => {
                if self.position.x + 1 < self.board_width {
                    return Position {
                        x: self.position.x + 1,
                        y: self.position.y,
                        orientation: self.position.orientation,
                    };
                } else {
                    return Position {
                        x: 0,
                        y: self.position.y,
                        orientation: self.position.orientation,
                    };
                }
            }
            Orientation::Left => {
                if self.position.x >= 1 {
                    return Position {
                        x: self.position.x - 1,
                        y: self.position.y,
                        orientation: self.position.orientation,
                    };
                } else {
                    return Position {
                        x: self.board_width - 1,
                        y: self.position.y,
                        orientation: self.position.orientation,
                    };
                }
            }
            Orientation::Up => {
                if self.position.y >= 1 {
                    return Position {
                        y: self.position.y - 1,
                        x: self.position.x,
                        orientation: self.position.orientation,
                    };
                } else {
                    return Position {
                        y: self.board_height - 1,
                        x: self.position.x,
                        orientation: self.position.orientation,
                    };
                }
            }
            Orientation::Down => {
                if self.position.y + 1 < self.board_height {
                    return Position {
                        y: self.position.y + 1,
                        x: self.position.x,
                        orientation: self.position.orientation,
                    };
                } else {
                    return Position {
                        y: 0,
                        x: self.position.x,
                        orientation: self.position.orientation,
                    };
                }
            }
        }
    }
    fn get_next_tile(&self) -> Tile {
        let next_position = self.get_next_position();
        let next_tile = self.board[next_position.y][next_position.x];
        return next_tile;
    }
    fn move_to_next_tile(&mut self) {
        let next_position = self.get_next_position();
        self.position = next_position;
    }
}

impl FromStr for Game {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (map, moves) = s.split_once("\n\n").unwrap();
        let height = map.lines().count();
        let width = map.lines().map(|line| line.len()).max().unwrap_or(0);
        let mut raw_board = vec![vec![Tile::Nothing; width]; height];
        for (i, line) in map.lines().enumerate() {
            for (j, tile) in line.chars().enumerate() {
                raw_board[i][j] = tile.into();
            }
        }
        let move_regex = Regex::new(r"(.*)(R|L)").unwrap();
        let split_moves: Vec<Move> = moves
            .split_inclusive(&['L', 'R'])
            .flat_map(|move_plus_rotation| {
                let matches = match move_regex.captures(move_plus_rotation) {
                    None => {
                        vec![Move::Amount(
                            move_plus_rotation.trim().parse::<usize>().unwrap(),
                        )]
                    }
                    Some(matches) => {
                        let amount = matches.get(1).unwrap().as_str();
                        let rotation_str = matches.get(2).unwrap().as_str().chars().nth(0).unwrap();
                        return vec![
                            Move::Amount(amount.parse().unwrap()),
                            Move::Turn(rotation_str.into()),
                        ];
                    }
                };
                return matches;
            })
            .collect();
        let x = raw_board
            .iter()
            .nth(0)
            .unwrap_or(&vec![Tile::Open])
            .iter()
            .position(|tile| *tile == Tile::Open)
            .unwrap();
        return Ok(Self {
            board_width: width,
            board_height: height,
            board: raw_board,
            moves: split_moves,
            position: Position {
                x,
                y: 0,
                orientation: Orientation::Right,
            },
        });
    }
}

impl From<char> for Rotation {
    fn from(c: char) -> Self {
        match c {
            'R' => Rotation::Clockwise,
            'L' => Rotation::CounterClockwise,
            _ => unreachable!(),
        }
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '#' => Tile::Wall,
            '.' => Tile::Open,
            ' ' => Tile::Nothing,
            _ => unreachable!(),
        }
    }
}

fn part_1(input: &str) -> usize {
    let mut game: Game = input.parse().unwrap();
    for game_move in game.moves.clone() {
        match game_move {
            Move::Turn(turn) => game.turn(&turn),
            Move::Amount(amount) => {
                for _ in 0..amount {
                    let mut next_tile = game.get_next_tile();
                    let pre_lookahead_position = game.position.clone();
                    while next_tile == Tile::Nothing {
                        game.move_to_next_tile();
                        next_tile = game.get_next_tile();
                    }
                    if next_tile == Tile::Wall {
                        game.position = pre_lookahead_position;
                        break;
                    }
                    game.move_to_next_tile();
                }
            }
        }
    }
    println!(
        "We finished at row: {:?}, column: {:?}, facing {:?}",
        game.position.y + 1,
        game.position.x + 1,
        game.position.orientation
    );
    let row = game.position.y + 1;
    let col = game.position.x + 1;
    let orientation_score = match game.position.orientation {
        Orientation::Right => 0,
        Orientation::Down => 1,
        Orientation::Left => 2,
        Orientation::Up => 3,
    };
    return (row * 1000 + 4 * col + orientation_score) as usize;
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn part_1_given() {
        let input = include_str!("../input.test");

        let output = part_1(input);

        assert_eq!(output, 6032);
    }
    #[test]
    fn parse_basic_board() {
        let input = " .#\n.#.\n\n";
        let expected_board = vec![
            vec![Tile::Nothing, Tile::Open, Tile::Wall],
            vec![Tile::Open, Tile::Wall, Tile::Open],
        ];

        let actual_board = input.parse::<Game>().unwrap().board;

        assert_eq!(expected_board, actual_board);
    }

    #[test]
    fn parse_complex_board() {
        let input = " .\n.#.\n\n";
        let expected_board = vec![
            vec![Tile::Nothing, Tile::Open, Tile::Nothing],
            vec![Tile::Open, Tile::Wall, Tile::Open],
        ];

        let actual_board = input.parse::<Game>().unwrap().board;

        assert_eq!(expected_board, actual_board);
    }

    #[test]
    fn basic_moves() {
        let input = "\n\n10L100R";
        let expected_moves = vec![
            Move::Amount(10),
            Move::Turn(Rotation::CounterClockwise),
            Move::Amount(100),
            Move::Turn(Rotation::Clockwise),
        ];

        let actual_moves = input.parse::<Game>().unwrap().moves;

        assert_eq!(expected_moves, actual_moves);
    }

    #[test]
    fn real_moves() {
        let input = "\n\n10L100R20";
        let expected_moves = vec![
            Move::Amount(10),
            Move::Turn(Rotation::CounterClockwise),
            Move::Amount(100),
            Move::Turn(Rotation::Clockwise),
            Move::Amount(20),
        ];

        let actual_moves = input.parse::<Game>().unwrap().moves;

        assert_eq!(expected_moves, actual_moves);
    }

    #[test]
    fn starting_position() {
        let input = " .#\n\n";
        let expected_position = Position {
            x: 1,
            y: 0,
            orientation: Orientation::Right,
        };

        let actual_position = input.parse::<Game>().unwrap().position;

        assert_eq!(expected_position, actual_position);
    }
}
