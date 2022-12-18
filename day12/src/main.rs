use std::collections::{HashSet, VecDeque};
use std::hash::Hash;
use std::str::FromStr;

use anyhow::Result;
fn main() {
    println!("Hello, world!");
    let step_count = part_2(include_str!("input.real")).unwrap();
    println!("Took {step_count} steps");
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Position {
    height: Height,
    x: i32,
    y: i32,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Height {
    Start,
    End,
    Normal(char),
}

struct HeightMap {
    map: Vec<Vec<Position>>,
    path_stack: VecDeque<Path>,
    considered_paths: Vec<Path>,
    finished_paths: Vec<Path>,
}

#[derive(Clone, Debug)]
struct Path {
    history: HashSet<Position>,
    current: Position,
}

impl Path {
    fn new(current: Position) -> Self {
        return Self {
            history: HashSet::new(),
            current,
        };
    }
    fn length(&self) -> usize {
        return self.history.len();
    }
    fn add_new_position(&self, new_position: Position) -> Option<Self> {
        let mut new_history = self.history.clone();
        new_history.insert(self.current);
        if !self.current.height.can_move_to(&new_position.height) {
            return None;
        }
        return Some(Self {
            history: new_history,
            current: new_position,
        });
    }
    fn is_finished(&self) -> bool {
        match self.current.height {
            Height::End => true,
            _ => false,
        }
    }
}

impl FromStr for Height {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Ok(match s {
            "S" => Height::Start,
            "E" => Height::End,
            _ => Height::Normal(s.chars().nth(0).unwrap()),
        });
    }
}

impl FromStr for HeightMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map: Vec<Vec<Position>> = s
            .lines()
            .enumerate()
            .map(|(x, line)| {
                return line
                    .chars()
                    .enumerate()
                    .map(|(y, height)| {
                        let height: Height = height.to_string().parse().unwrap();
                        let x = x as i32;
                        let y = y as i32;
                        return Position { height, x, y };
                    })
                    .collect();
            })
            .collect();

        let stack = VecDeque::new();

        return Ok(Self {
            map,
            path_stack: stack,
            considered_paths: Vec::new(),
            finished_paths: Vec::new(),
        });
    }
}

impl HeightMap {
    fn initialize_s_start(&mut self) {
        let starting_position: Position = self
            .map
            .iter()
            .find_map(|row| return row.iter().find(|position| position.height == Height::Start))
            .unwrap()
            .clone();
        self.path_stack.push_back(Path::new(starting_position));
    }
    fn initialize_a_start(&mut self) {
        let starting_positions: Vec<&Position> = self
            .map
            .iter()
            .flat_map(|row| {
                return row.iter().filter(|position| match position.height {
                    Height::Start => true,
                    Height::End => false,
                    Height::Normal(height) => match height {
                        'a' => true,
                        _ => false,
                    },
                });
            })
            .collect();
        for starting_position in starting_positions {
            self.path_stack.push_back(Path::new(*starting_position));
        }
    }
    fn next_path(&mut self) -> Option<Path> {
        let path = self.path_stack.pop_front()?;
        self.considered_paths.push(path.clone());
        return Some(path);
    }
    fn all_valid_new_paths(&self, base: Path) -> Vec<Path> {
        let option_positions = vec![
            self.position_at(base.current.x - 1, base.current.y),
            self.position_at(base.current.x + 1, base.current.y),
            self.position_at(base.current.x, base.current.y - 1),
            self.position_at(base.current.x, base.current.y + 1),
        ];
        //println!("OPtion Positions are {:?}", option_positions);
        let all_valid_positions: Vec<&Position> = option_positions.iter().flat_map(|a| a).collect();
        let all_valid_paths: Vec<Path> = all_valid_positions
            .iter()
            .flat_map(|position| {
                return base.add_new_position(**position);
            })
            .filter(|new_path| {
                let shorter_path_has_been_considered =
                    self.considered_paths.iter().any(|existing_path| {
                        existing_path.current == new_path.current
                            && existing_path.length() <= new_path.length()
                    });
                return !shorter_path_has_been_considered;
            })
            .filter(|new_path| {
                let shorter_path_exists = self.path_stack.iter().any(|existing_path| {
                    existing_path.current == new_path.current
                        && existing_path.length() <= new_path.length()
                });
                return !shorter_path_exists;
            })
            .collect();
        //println!("Valid Paths are {:?}", all_valid_paths);
        return all_valid_paths;
    }
    fn position_at(&self, x: i32, y: i32) -> Option<Position> {
        if x < 0 || x >= self.map.len() as i32 {
            return None;
        }
        if y < 0 || y >= self.map[0].len() as i32 {
            return None;
        }
        return Some(self.map[x as usize][y as usize]);
    }
}

impl Height {
    fn convert_height(&self) -> i32 {
        return (match self {
            Height::Start => 'a',
            Height::End => 'z',
            Height::Normal(height) => *height,
        }) as i32;
    }
    fn can_move_to(&self, other: &Height) -> bool {
        let self_height = self.convert_height();
        let other_height = other.convert_height();
        return self_height - other_height >= -1;
    }
}

fn part_1(input: &str) -> Result<usize> {
    let mut height_map: HeightMap = input.parse().unwrap();
    height_map.initialize_s_start();
    let mut count = 0;
    while !height_map.path_stack.is_empty() {
        count = count + 1;
        //println!("on iter {count}");
        let path = height_map.next_path().unwrap();
        if path.is_finished() {
            height_map.finished_paths.push(path.clone());
            continue;
        }
        let new_paths = height_map.all_valid_new_paths(path);
        for new_path in new_paths {
            height_map.path_stack.push_back(new_path);
        }
    }
    let shortest_path: &Path = height_map
        .finished_paths
        .iter()
        .reduce(|accum, elem| {
            if elem.length() < accum.length() {
                return elem;
            } else {
                return accum;
            }
        })
        .unwrap();
    return Ok(shortest_path.length());
}

fn part_2(input: &str) -> Result<usize> {
    let mut height_map: HeightMap = input.parse().unwrap();
    height_map.initialize_a_start();
    let mut count = 0;
    while !height_map.path_stack.is_empty() {
        count = count + 1;
        //println!("on iter {count}");
        let path = height_map.next_path().unwrap();
        if path.is_finished() {
            height_map.finished_paths.push(path.clone());
            continue;
        }
        let new_paths = height_map.all_valid_new_paths(path);
        for new_path in new_paths {
            height_map.path_stack.push_back(new_path);
        }
    }
    let shortest_path: &Path = height_map
        .finished_paths
        .iter()
        .reduce(|accum, elem| {
            if elem.length() < accum.length() {
                return elem;
            } else {
                return accum;
            }
        })
        .unwrap();
    return Ok(shortest_path.length());
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn part1_given() {
        let input = include_str!("input.test");

        let result = part_1(input).unwrap();

        assert_eq!(result, 31);
    }

    #[test]
    fn part2_given() {
        let input = include_str!("input.test");

        let result = part_2(input).unwrap();

        assert_eq!(result, 29);
    }
}
