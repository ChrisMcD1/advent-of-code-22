use std::collections::HashSet;
use std::str::FromStr;
use std::time::Instant;

fn main() {
    println!("Hello, world!");
    let start = Instant::now();

    let input = include_str!("../input.prod");
    let result = part_1(input, 2_000_000);
    println!("Result is {result} in {:?}", start.elapsed());

    let result = part_2(input, 4_000_000);
    println!("Result is {result} in {:?}", start.elapsed());
}

fn part_2(input: &str, max_value: i64) -> i64 {
    let sensor_data: Vec<SensorData> = input.lines().map(|line| line.parse().unwrap()).collect();

    let possibilites: Vec<Coordinate> = sensor_data
        .iter()
        .flat_map(|sensor| sensor.circle_of_possibility())
        .filter(|coord| {
            coord.x >= 0 && coord.x <= max_value && coord.y >= 0 && coord.y <= max_value
        })
        .collect();

    let place = possibilites
        .iter()
        .find(|coord| {
            sensor_data
                .iter()
                .all(|sensor| !sensor.coordinate_not_beacon(coord))
        })
        .unwrap();

    place.x * 4_000_000 + place.y
}

fn part_1(input: &str, target_row: i64) -> usize {
    let sensor_data: Vec<SensorData> = input.lines().map(|line| line.parse().unwrap()).collect();

    let ranges: Vec<Range> = sensor_data
        .into_iter()
        .flat_map(|sensor| sensor.non_beacon_spaces_range(target_row))
        .collect();

    let lowest = ranges.iter().map(|range| range.lower).min().unwrap();
    let highest = ranges.iter().map(|range| range.upper).max().unwrap();

    ((lowest - 1)..(highest + 1))
        .filter(|&i| ranges.iter().any(|range| range.num_in_range(i)))
        .count()
}

#[derive(Debug, PartialEq, Hash, Eq)]
struct Coordinate {
    x: i64,
    y: i64,
}

impl FromStr for Coordinate {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (x_region, y_region) = input.split_once(',').unwrap();

        let (x, x_str) = x_region.split_once('=').unwrap();
        let (y, y_str) = y_region.split_once('=').unwrap();

        return Ok(Coordinate {
            x: x_str.parse().unwrap(),
            y: y_str.parse().unwrap(),
        });
    }
}

#[derive(Debug, PartialEq)]

struct SensorData {
    sensor: Coordinate,
    closest_beacon: Coordinate,
    manhattan_distance: i64,
}

impl FromStr for SensorData {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (sensor, beacon) = input.split_once(':').unwrap();

        Ok(SensorData::new(
            sensor.parse().unwrap(),
            beacon.parse().unwrap(),
        ))
    }
}

fn abs(value: i64) -> i64 {
    if value > 0 {
        return value;
    } else {
        return -1 * value;
    }
}

#[derive(Debug, PartialEq)]
struct Range {
    lower: i64,
    upper: i64,
}

impl Range {
    fn num_in_range(&self, num: i64) -> bool {
        num >= self.lower && num <= self.upper
    }
}

impl SensorData {
    fn new(sensor: Coordinate, closest_beacon: Coordinate) -> Self {
        let manhattan_distance =
            abs(sensor.x - closest_beacon.x) + abs(sensor.y - closest_beacon.y);
        Self {
            sensor,
            closest_beacon,
            manhattan_distance,
        }
    }

    fn coordinate_not_beacon(&self, coord: &Coordinate) -> bool {
        let coord_distance = abs(self.sensor.x - coord.x) + abs(self.sensor.y - coord.y);
        coord_distance <= self.manhattan_distance
    }
    fn circle_of_possibility(&self) -> Vec<Coordinate> {
        let distance = self.manhattan_distance + 1;
        let mut vec = Vec::with_capacity((distance * 4) as usize);
        // top left
        for i in 0..distance {
            vec.push(Coordinate {
                x: self.sensor.x - i,
                y: self.sensor.y + (distance - i),
            })
        }
        // top right
        for i in 0..distance {
            vec.push(Coordinate {
                x: self.sensor.x + i,
                y: self.sensor.y + (distance - i),
            })
        }
        // bottom right
        for i in 0..distance {
            vec.push(Coordinate {
                x: self.sensor.x - i,
                y: self.sensor.y - (distance - i),
            })
        }
        // bottom left
        for i in 0..distance {
            vec.push(Coordinate {
                x: self.sensor.x + i,
                y: self.sensor.y - (distance - i),
            })
        }
        vec
    }

    fn non_beacon_spaces_range(&self, target_row: i64) -> Option<Range> {
        let manhattan_distance =
            abs(self.sensor.x - self.closest_beacon.x) + abs(self.sensor.y - self.closest_beacon.y);
        let y_offset = abs(target_row - self.sensor.y);
        if abs(y_offset) > manhattan_distance {
            return None;
        }

        let x_offset = manhattan_distance - y_offset;

        let mut range = Range {
            lower: self.sensor.x - x_offset,
            upper: self.sensor.x + x_offset,
        };

        if self.closest_beacon.y == target_row {
            if self.closest_beacon.x < self.sensor.x {
                range.lower = range.lower + 1;
            } else if self.closest_beacon.x > self.sensor.x {
                range.upper = range.upper - 1;
            } else {
                return None;
            }
        }

        return Some(range);
    }
    fn non_beacon_spaces(&self) -> HashSet<Coordinate> {
        let non_beacon_spaces: HashSet<Coordinate> = (-self.manhattan_distance
            ..=self.manhattan_distance)
            .flat_map(|x_offset| {
                let y_offset = self.manhattan_distance - abs(x_offset);
                return (-y_offset..=y_offset).map(move |y| Coordinate {
                    x: self.sensor.x + x_offset,
                    y: self.sensor.y + y,
                });
            })
            .collect();

        return non_beacon_spaces;
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn part_1_given() {
        let input = include_str!("../input.dev");

        let output = part_1(input, 10);

        assert_eq!(output, 26);
    }

    #[test]
    fn part_2_given() {
        let input = include_str!("../input.dev");

        let output = part_2(input, 20);

        assert_eq!(output, 56000011);
    }

    #[test]
    fn parse_coordinate_sensor() {
        let input = "Sensor at x=2, y=18";
        let expected_coordinate = Coordinate { x: 2, y: 18 };

        let actual_coordinate: Coordinate = input.parse().unwrap();

        assert_eq!(expected_coordinate, actual_coordinate);
    }

    #[test]

    fn parse_coordinate_beacon() {
        let input = "closest beacon is at x=-2, y=15";

        let expected_coordinate = Coordinate { x: -2, y: 15 };

        let actual_coordinate: Coordinate = input.parse().unwrap();

        assert_eq!(expected_coordinate, actual_coordinate);
    }
    #[test]

    fn parse_sensor_data() {
        let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15";

        let expected_sensor =
            SensorData::new(Coordinate { x: 2, y: 18 }, Coordinate { x: -2, y: 15 });
        let actual_sensor: SensorData = input.parse().unwrap();
        assert_eq!(expected_sensor, actual_sensor);
    }

    #[test]

    fn find_blocked_spaces() {
        let sensor_data = SensorData::new(Coordinate { x: 0, y: 0 }, Coordinate { x: 1, y: 0 });

        let mut expected_blocked_coordinates: HashSet<Coordinate> = HashSet::new();

        expected_blocked_coordinates.insert(Coordinate { x: -1, y: 0 });
        expected_blocked_coordinates.insert(Coordinate { x: 0, y: 0 });
        expected_blocked_coordinates.insert(Coordinate { x: 0, y: -1 });
        expected_blocked_coordinates.insert(Coordinate { x: 0, y: 1 });
        expected_blocked_coordinates.insert(Coordinate { x: 1, y: 0 });

        let actual_blocked_coordinates = sensor_data.non_beacon_spaces();
        assert_eq!(expected_blocked_coordinates, actual_blocked_coordinates);
    }

    #[test]
    fn range_upper_beacon() {
        let sensor_data = SensorData::new(Coordinate { x: 0, y: 0 }, Coordinate { x: 2, y: 3 });
        let expected_range = Range {
            lower: -2,
            upper: 1,
        };

        let range = sensor_data.non_beacon_spaces_range(3).unwrap();

        assert_eq!(expected_range, range);
    }

    #[test]
    fn range_lower_beacon() {
        let sensor_data = SensorData::new(Coordinate { x: 0, y: 0 }, Coordinate { x: -2, y: 3 });
        let expected_range = Range {
            lower: -1,
            upper: 2,
        };

        let range = sensor_data.non_beacon_spaces_range(3).unwrap();

        assert_eq!(expected_range, range);
    }

    #[test]
    fn range_no_conflict() {
        let sensor_data = SensorData::new(Coordinate { x: 0, y: 0 }, Coordinate { x: -2, y: 3 });
        let expected_range = Range {
            lower: -3,
            upper: 3,
        };

        let range = sensor_data.non_beacon_spaces_range(2).unwrap();

        assert_eq!(expected_range, range);
    }

    #[test]
    fn range_given() {
        let sensor_data = SensorData::new(Coordinate { x: 8, y: 7 }, Coordinate { x: 2, y: 10 });
        let expected_range = Range {
            lower: 3,
            upper: 14,
        };

        let range = sensor_data.non_beacon_spaces_range(10).unwrap();

        assert_eq!(expected_range, range);
    }
}
