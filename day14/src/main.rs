use std::str::FromStr;

use anyhow::Result;
fn main() {
    println!("Hello, world!");
    let input = include_str!("input.test");
    let result = part_2(input).unwrap();
    println!("Planted {result} sand");
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Coordinate {
    x: i64,
    y: i64,
}

impl FromStr for Coordinate {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').unwrap();
        println!("x: {x}, y: {y}");
        return Ok(Self {
            x: x.trim().parse().unwrap(),
            y: y.trim().parse().unwrap(),
        });
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CellContents {
    Air,
    Rock,
    Sand,
}

#[derive(Debug)]
struct Landscape {
    contents: Vec<Vec<CellContents>>,
}

impl Landscape {
    fn new(size: usize) -> Self {
        return Landscape {
            contents: vec![vec![CellContents::Air; size]; size],
        };
    }
}

fn part_2(input: &str) -> Result<usize> {
    let mut landscape = Landscape::new(1000);
    let rocks = parse_rocks(input);
    fill_landscape_with_rocks(&mut landscape, rocks);
    let max_y_count = landscape
        .contents
        .iter()
        .map(|col| {
            return 999
                - col
                    .iter()
                    .rev()
                    .position(|&cell| cell == CellContents::Rock)
                    .unwrap_or(999);
        })
        .max()
        .unwrap();

    println!("Max Y is at {max_y_count}");
    let bottom_rock = vec![
        Coordinate {
            x: 0,
            y: max_y_count as i64 + 2,
        },
        Coordinate {
            x: 999,
            y: max_y_count as i64 + 2,
        },
    ];
    fill_landscape_with_rocks(&mut landscape, vec![bottom_rock]);
    let mut sand_full = false;
    while !sand_full {
        let mut sand_x = 500;
        let mut sand_y = 0;
        sand_full = landscape.contents[sand_x][sand_y] == CellContents::Sand;
        let mut sand_resting = false;
        while !sand_resting && !sand_full {
            if landscape.contents[sand_x][sand_y + 1] == CellContents::Air {
                sand_y = sand_y + 1;
            } else if landscape.contents[sand_x - 1][sand_y + 1] == CellContents::Air {
                sand_y = sand_y + 1;
                sand_x = sand_x - 1;
            } else if landscape.contents[sand_x + 1][sand_y + 1] == CellContents::Air {
                sand_y = sand_y + 1;
                sand_x = sand_x + 1;
            } else {
                landscape.contents[sand_x][sand_y] = CellContents::Sand;
                sand_resting = true;
            }
        }
    }
    let sand_count: usize = landscape
        .contents
        .iter()
        .map(|row| {
            row.iter()
                .filter(|&cell| match &cell {
                    CellContents::Sand => true,
                    _ => false,
                })
                .count()
        })
        .sum();
    return Ok(sand_count);
}

fn part_1(input: &str) -> Result<usize> {
    let mut landscape = Landscape::new(1000);
    let rocks = parse_rocks(input);
    fill_landscape_with_rocks(&mut landscape, rocks);

    let mut sand_overflowing = false;
    while !sand_overflowing {
        let mut sand_x = 500;
        let mut sand_y = 0;
        let mut sand_resting = false;
        while !sand_resting && !sand_overflowing {
            if landscape.contents[sand_x][sand_y + 1] == CellContents::Air {
                sand_y = sand_y + 1;
            } else if landscape.contents[sand_x - 1][sand_y + 1] == CellContents::Air {
                sand_y = sand_y + 1;
                sand_x = sand_x - 1;
            } else if landscape.contents[sand_x + 1][sand_y + 1] == CellContents::Air {
                sand_y = sand_y + 1;
                sand_x = sand_x + 1;
            } else {
                landscape.contents[sand_x][sand_y] = CellContents::Sand;
                sand_resting = true;
            }
            if sand_y > 998 {
                sand_overflowing = true;
            }
        }
    }
    let sand_count: usize = landscape
        .contents
        .iter()
        .map(|row| {
            row.iter()
                .filter(|&cell| match &cell {
                    CellContents::Sand => true,
                    _ => false,
                })
                .count()
        })
        .sum();
    return Ok(sand_count);
}

fn parse_rocks(input: &str) -> Vec<Vec<Coordinate>> {
    let rocks: Vec<Vec<Coordinate>> = input
        .lines()
        .map(|line| {
            return line
                .split("->")
                .map(|coord| coord.parse::<Coordinate>().unwrap())
                .collect();
        })
        .collect();
    return rocks;
}

fn fill_landscape_with_rocks(landscape: &mut Landscape, rocks: Vec<Vec<Coordinate>>) {
    for rock in rocks {
        rock.windows(2).for_each(|window| {
            let first = window[0];
            let second = window[1];
            let x_range = second.x - first.x;
            let y_range = second.y - first.y;
            for x in first.x..=first.x + x_range {
                landscape.contents[x as usize][first.y as usize] = CellContents::Rock;
            }
            for x in first.x + x_range..=first.x {
                landscape.contents[x as usize][first.y as usize] = CellContents::Rock;
            }
            for y in first.y..=first.y + y_range {
                landscape.contents[first.x as usize][y as usize] = CellContents::Rock;
            }
            for y in first.y + y_range..=first.y {
                landscape.contents[first.x as usize][y as usize] = CellContents::Rock;
            }
        });
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn part1_given() {
        let input = include_str!("input.given");

        let result = part_1(input).unwrap();

        assert_eq!(result, 24);
    }

    #[test]
    fn part2_given() {
        let input = include_str!("input.given");

        let result = part_2(input).unwrap();

        assert_eq!(result, 93);
    }

    #[test]
    fn parse_rocks_one_line() {
        let rocks_str = "498,4 -> 498,6 -> 496,6";

        let rocks = parse_rocks(rocks_str);

        assert_eq!(
            rocks,
            vec![vec![
                Coordinate { x: 498, y: 4 },
                Coordinate { x: 498, y: 6 },
                Coordinate { x: 496, y: 6 }
            ]]
        );
    }

    #[test]
    fn parse_rocks_longer() {
        let rocks_str = "503,4 -> 502,4 -> 502,9 -> 494,9";

        let rocks = parse_rocks(rocks_str);

        assert_eq!(
            rocks,
            vec![vec![
                Coordinate { x: 503, y: 4 },
                Coordinate { x: 502, y: 4 },
                Coordinate { x: 502, y: 9 },
                Coordinate { x: 494, y: 9 }
            ]]
        );
    }

    #[test]
    fn parse_rocks_2_lines() {
        let rocks_str = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9";

        let rocks = parse_rocks(rocks_str);

        assert_eq!(
            rocks,
            vec![
                vec![
                    Coordinate { x: 498, y: 4 },
                    Coordinate { x: 498, y: 6 },
                    Coordinate { x: 496, y: 6 }
                ],
                vec![
                    Coordinate { x: 503, y: 4 },
                    Coordinate { x: 502, y: 4 },
                    Coordinate { x: 502, y: 9 },
                    Coordinate { x: 494, y: 9 }
                ]
            ]
        );
    }
    #[test]
    fn fill_rocks_1D() {
        let rocks = vec![vec![Coordinate { x: 0, y: 0 }, Coordinate { x: 0, y: 5 }]];
        let mut landscape = Landscape::new(10);

        fill_landscape_with_rocks(&mut landscape, rocks);
        let rock_slice = &landscape.contents[0][0..=5];

        assert_eq!(
            rock_slice,
            &[
                CellContents::Rock,
                CellContents::Rock,
                CellContents::Rock,
                CellContents::Rock,
                CellContents::Rock,
                CellContents::Rock
            ]
        );
    }

    #[test]
    fn fill_rocks_2D() {
        let rocks = vec![vec![
            Coordinate { x: 0, y: 0 },
            Coordinate { x: 0, y: 5 },
            Coordinate { x: 2, y: 5 },
        ]];
        let mut landscape = Landscape::new(10);

        fill_landscape_with_rocks(&mut landscape, rocks);
        let vertical_rock_slice = &landscape.contents[0][0..=5];
        let horizontal_rock_point_1 = &landscape.contents[1][5];
        let horizontal_rock_point_2 = &landscape.contents[2][5];

        assert_eq!(
            vertical_rock_slice,
            &[
                CellContents::Rock,
                CellContents::Rock,
                CellContents::Rock,
                CellContents::Rock,
                CellContents::Rock,
                CellContents::Rock
            ]
        );
        assert_eq!(horizontal_rock_point_1, &CellContents::Rock);
        assert_eq!(horizontal_rock_point_2, &CellContents::Rock);
    }
}
