use anyhow::anyhow;
use std::time::Instant;

fn main() {
    println!("Hello, world!");
    let start = Instant::now();
    let input = include_str!("../input.prod");
    let piece_count = 100_000;
    let output = solve(input, piece_count);

    println!(
        "Got {output} for {piece_count} pieces in {:?}",
        start.elapsed()
    );
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Space {
    Empty,
    Full,
}

enum Push {
    Left,
    Right,
}

impl TryFrom<char> for Push {
    type Error = anyhow::Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '<' => Ok(Self::Left),
            '>' => Ok(Self::Right),
            _ => Err(anyhow!("Bad!")),
        }
    }
}

#[derive(Debug)]
struct Coordinate {
    x: i64,
    y: i64,
}

struct Chamber {
    spaces: Vec<Vec<Space>>,
    height: i64,
}

impl Chamber {
    fn new() -> Self {
        let height = 100_000;
        let mut spaces = vec![vec![Space::Empty; 9]; height as usize];
        spaces[0] = vec![Space::Full; 9];
        for row in spaces.iter_mut() {
            row[0] = Space::Full;
            row[8] = Space::Full;
        }
        Self { spaces, height }
    }
    fn set_space(&mut self, y: i64, x: i64, space: Space) {
        self.spaces[(y.rem_euclid(self.height)) as usize][x as usize] = space;
    }
    fn get_space(&self, y: i64, x: i64) -> Space {
        self.spaces[(y.rem_euclid(self.height)) as usize][x as usize]
    }
    fn clear_rows_above(&mut self, height: i64) {
        for i in 1..10 {
            for x in 1..=7 {
                self.set_space(height + i, x, Space::Empty);
            }
        }
    }
    fn print_first_10(&self) {
        let first_10 = &self.spaces[0..10];
        println!("{:#?}", first_10);
    }
    fn max_height(&self) -> i64 {
        self.spaces
            .iter()
            .position(|row| {
                row[1..=7].iter().all(|space| match space {
                    Space::Empty => true,
                    _ => false,
                })
            })
            .expect("At least one run must have all empty spaces")
            .try_into()
            .unwrap()
    }
    fn place_piece(&mut self, piece: &Vec<Coordinate>, coord: &Coordinate) {
        for part in piece.iter() {
            let absolute_x = coord.x + part.x;
            let absolute_y = coord.y + part.y;
            self.set_space(absolute_y, absolute_x, Space::Full);
        }

        let tower_max = self.max_height();

        self.clear_rows_above(coord.y + tower_max);
    }

    fn any_pieces_below(&self, piece: &Vec<Coordinate>, coord: &Coordinate) -> bool {
        piece.iter().any(|relative| {
            let x = coord.x + relative.x;
            let y = (coord.y + relative.y) - 1;
            match self.get_space(y, x) {
                Space::Full => true,
                Space::Empty => false,
            }
        })
    }

    fn any_pieces_to_left(&self, piece: &Vec<Coordinate>, coord: &Coordinate) -> bool {
        piece.iter().any(|relative| {
            let x = coord.x + relative.x - 1;
            let y = coord.y + relative.y;
            match self.get_space(y, x) {
                Space::Full => true,
                Space::Empty => false,
            }
        })
    }

    fn any_pieces_to_right(&self, piece: &Vec<Coordinate>, coord: &Coordinate) -> bool {
        piece.iter().any(|relative| {
            let x = coord.x + relative.x + 1;
            let y = coord.y + relative.y;
            match self.get_space(y, x) {
                Space::Full => true,
                Space::Empty => false,
            }
        })
    }

    fn push_piece(&self, piece: &Vec<Coordinate>, coord: Coordinate, push: &Push) -> Coordinate {
        match push {
            Push::Left => {
                if self.any_pieces_to_left(piece, &coord) {
                    coord
                } else {
                    Coordinate {
                        x: coord.x - 1,
                        y: coord.y,
                    }
                }
            }
            Push::Right => {
                if self.any_pieces_to_right(piece, &coord) {
                    coord
                } else {
                    Coordinate {
                        x: coord.x + 1,
                        y: coord.y,
                    }
                }
            }
        }
    }
}

fn solve(input: &str, piece_count: usize) -> i64 {
    let pushes: Vec<Push> = input.chars().flat_map(|c| c.try_into()).collect();
    let mut chamber = Chamber::new();
    let minus = vec![
        Coordinate { x: 0, y: 0 },
        Coordinate { x: 1, y: 0 },
        Coordinate { x: 2, y: 0 },
        Coordinate { x: 3, y: 0 },
    ];

    let plus = vec![
        Coordinate { x: 1, y: 0 },
        Coordinate { x: 0, y: 1 },
        Coordinate { x: 1, y: 1 },
        Coordinate { x: 2, y: 1 },
        Coordinate { x: 1, y: 2 },
    ];

    let l = vec![
        Coordinate { x: 0, y: 0 },
        Coordinate { x: 1, y: 0 },
        Coordinate { x: 2, y: 0 },
        Coordinate { x: 2, y: 1 },
        Coordinate { x: 2, y: 2 },
    ];

    let vertical = vec![
        Coordinate { x: 0, y: 0 },
        Coordinate { x: 0, y: 1 },
        Coordinate { x: 0, y: 2 },
        Coordinate { x: 0, y: 3 },
    ];

    let square = vec![
        Coordinate { x: 0, y: 0 },
        Coordinate { x: 0, y: 1 },
        Coordinate { x: 1, y: 0 },
        Coordinate { x: 1, y: 1 },
    ];

    let pieces = vec![minus, plus, l, vertical, square];
    let mut pushes_count = 0;
    for num in 0..piece_count {
        let starting_height = chamber.max_height() + 3;
        let mut coord = Coordinate {
            x: 3,
            y: starting_height,
        };

        let piece = &pieces[num % pieces.len()];
        loop {
            let push = &pushes[pushes_count % pushes.len()];
            pushes_count = pushes_count + 1;
            coord = chamber.push_piece(piece, coord, push);

            // time to try to push
            if chamber.any_pieces_below(piece, &coord) {
                break;
            } else {
                coord.y = coord.y - 1;
            }
        }
        chamber.place_piece(piece, &coord);
    }
    chamber.max_height() - 1
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn part_1_given() {
        let input = include_str!("../input.dev");

        let output = solve(input, 2022);

        assert_eq!(output, 3068);
    }

    #[test]
    #[ignore]
    fn part_2_given() {
        let input = include_str!("../input.dev");
        let output = solve(input, 1_000_000_000_000);

        assert_eq!(output, 1514285714288);
    }
}
