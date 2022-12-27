use std::str::FromStr;

fn main() {
    println!("Hello, world!");
    let input = include_str!("../input.prod");
    let all_edges = part_1(input);
    let air_spaces = part_2(input);
    println!("{:?}", all_edges - air_spaces);
}
fn part_2(input: &str) -> usize {
    let blocks: Vec<Coordinate> = input.lines().map(|line| line.parse().unwrap()).collect();
    let max_x = blocks.iter().map(|b| b.x).max().unwrap();
    let max_y = blocks.iter().map(|b| b.y).max().unwrap();
    let max_z = blocks.iter().map(|b| b.z).max().unwrap();
    let mut space = Space::new(
        (max_x + 1) as usize,
        (max_y + 1) as usize,
        (max_z + 1) as usize,
    );
    space.add_blocks(&blocks);
    space.flood();
    let air_spaces: Vec<Coordinate> = space
        .all
        .iter()
        .enumerate()
        .flat_map(move |(x, yz)| {
            yz.iter().enumerate().flat_map(move |(y, z_vec)| {
                z_vec
                    .iter()
                    .enumerate()
                    .filter(|(_, point)| match point {
                        Point::Air => true,
                        _ => false,
                    })
                    .map(move |(z, _)| Coordinate {
                        x: x.try_into().unwrap(),
                        y: y.try_into().unwrap(),
                        z: z.try_into().unwrap(),
                    })
            })
        })
        .collect();
    let mut exposed_edges = air_spaces.len() * 6;
    air_spaces.iter().for_each(|cube| {
        let other_cubes = air_spaces.iter().filter(|&other| other != cube);
        let covered_sides = other_cubes
            .filter(|other| cube.any_faces_touch(other))
            .count();
        exposed_edges = exposed_edges - covered_sides;
    });
    exposed_edges
}

fn part_1(input: &str) -> usize {
    let cubes: Vec<Coordinate> = input.lines().map(|line| line.parse().unwrap()).collect();
    let mut exposed_edges = cubes.len() * 6;
    cubes.iter().for_each(|cube| {
        let other_cubes = cubes.iter().filter(|&other| other != cube);
        let covered_sides = other_cubes
            .filter(|other| cube.any_faces_touch(other))
            .count();
        exposed_edges = exposed_edges - covered_sides;
    });
    exposed_edges
}
struct Space {
    all: Vec<Vec<Vec<Point>>>,
}

impl Space {
    fn new(x: usize, y: usize, z: usize) -> Self {
        Self {
            all: vec![vec![vec![Point::Air; z]; y]; x],
        }
    }
    fn x_length(&self) -> i64 {
        self.all.len() as i64
    }
    fn y_length(&self) -> i64 {
        self.all[0].len() as i64
    }
    fn z_length(&self) -> i64 {
        self.all[0][0].len() as i64
    }
    fn add_blocks(&mut self, blocks: &Vec<Coordinate>) {
        for block in blocks.iter() {
            self.all[block.x as usize][block.y as usize][block.z as usize] = Point::Block;
        }
    }
    fn flood(&mut self) {
        let mut coordinates = vec![Coordinate { x: 0, y: 0, z: 0 }];
        while let Some(c) = coordinates.pop() {
            if c.x < 0
                || c.y < 0
                || c.z < 0
                || c.x == self.x_length()
                || c.y == self.y_length()
                || c.z == self.z_length()
            {
                continue;
            }
            let point = &self.all[c.x as usize][c.y as usize][c.z as usize];
            if let Point::Air = point {
                self.all[c.x as usize][c.y as usize][c.z as usize] = Point::Water;
                coordinates.push(Coordinate {
                    x: c.x + 1,
                    y: c.y,
                    z: c.z,
                });
                coordinates.push(Coordinate {
                    x: c.x - 1,
                    y: c.y,
                    z: c.z,
                });
                coordinates.push(Coordinate {
                    x: c.x,
                    y: c.y + 1,
                    z: c.z,
                });
                coordinates.push(Coordinate {
                    x: c.x,
                    y: c.y - 1,
                    z: c.z,
                });
                coordinates.push(Coordinate {
                    x: c.x,
                    y: c.y,
                    z: c.z + 1,
                });
                coordinates.push(Coordinate {
                    x: c.x,
                    y: c.y,
                    z: c.z - 1,
                });
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Point {
    Water,
    Block,
    Air,
}

#[derive(Debug, PartialEq)]
struct Coordinate {
    x: i64,
    y: i64,
    z: i64,
}

impl Coordinate {
    fn any_faces_touch(&self, other: &Self) -> bool {
        return (self.y == other.y && self.z == other.z && (self.x - other.x).abs() == 1)
            || (self.x == other.x && self.z == other.z && (self.y - other.y).abs() == 1)
            || (self.x == other.x && self.y == other.y && (self.z - other.z).abs() == 1);
    }
}

impl FromStr for Coordinate {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split(",").collect();
        Ok(Self {
            x: coords[0].parse().unwrap(),
            y: coords[1].parse().unwrap(),
            z: coords[2].parse().unwrap(),
        })
    }
}
