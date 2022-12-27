use std::{collections::VecDeque, str::FromStr};

fn main() {
    println!("Hello, world!");
    let input = include_str!("../input.prod");
    let output = part_2(input);
    println!("Output is {output}");
}

fn part_1(input: &str) -> i64 {
    let mut circular_vec: CircularVec = input.parse().unwrap();
    circular_vec.swap_all_items(1);
    //println!("{circular_vec:?}");
    circular_vec.score()
}

fn part_2(input: &str) -> i64 {
    let mut circular_vec: CircularVec = input.parse().unwrap();
    circular_vec
        .data
        .iter_mut()
        .for_each(|elem| elem.value = elem.value * 811589153);
    circular_vec.swap_all_items(10);
    circular_vec.score()
}

#[derive(Debug, Clone, PartialEq, Copy)]
struct UniqueNumber {
    value: i64,
    starting_index: usize,
}

#[derive(Debug, PartialEq)]
struct CircularVec {
    data: VecDeque<UniqueNumber>,
}

impl CircularVec {
    fn score(&self) -> i64 {
        let zero_index = self.data.iter().position(|&num| num.value == 0).unwrap();
        let values = vec![
            self.at(zero_index + 1000),
            self.at(zero_index + 2000),
            self.at(zero_index + 3000),
        ];
        values.iter().sum()
    }
    fn at(&self, index: usize) -> i64 {
        self.data[index % self.data.len()].value
    }
    fn swap_forward(&mut self, index: usize) -> usize {
        if index == self.data.len() - 1 {
            let ahead_data = self.data[0];
            self.data[0] = self.data[index];
            self.data[index] = ahead_data;
            0
        } else {
            let ahead_data = self.data[index + 1];
            self.data[index + 1] = self.data[index];
            self.data[index] = ahead_data;
            index + 1
        }
    }
    fn swap_backward(&mut self, index: usize) -> usize {
        if index == 0 {
            let new_index = self.data.len() - 1;
            let behind_data = self.data[new_index];
            self.data[new_index] = self.data[0];
            self.data[0] = behind_data;
            new_index
        } else {
            let new_index = index - 1;
            let behind_data = self.data[new_index];
            self.data[new_index] = self.data[index];
            self.data[index] = behind_data;
            new_index
        }
    }
    fn swap_item(&mut self, val: UniqueNumber) {
        let mut index = self.data.iter().position(|&item| item == val).unwrap();
        let spaces_to_move = val.value % i64::try_from(self.data.len()).unwrap();
        let spaces_to_move = val.value % i64::try_from(self.data.len() - 1).unwrap();
        //let spaces_to_move = val.value;
        if spaces_to_move < 0 {
            println!("Entered neg with {spaces_to_move} from {:?}", val.value);
            for _ in 0..-spaces_to_move {
                index = self.swap_backward(index);
            }
        } else if spaces_to_move > 0 {
            println!("Entered pos with {spaces_to_move} from {:?}", val.value);
            for _ in 0..spaces_to_move {
                index = self.swap_forward(index);
            }
        }
    }
    fn swap_all_items(&mut self, count: usize) {
        let data_clone = self.data.clone();
        //println!("Initial is {data_clone:#?}");
        for i in 0..count {
            for item in data_clone.iter() {
                self.swap_item(*item);
            }
            //println!("After {i} rounds of mixing: {self:#?}");
        }
    }
}

impl FromStr for CircularVec {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: VecDeque<UniqueNumber> = s
            .lines()
            .enumerate()
            .map(|(i, line)| UniqueNumber {
                value: line.parse().unwrap(),
                starting_index: i,
            })
            .collect();
        Ok(CircularVec { data })
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn part_1_dev() {
        let input = include_str!("../input.dev");

        let output = part_1(input);

        assert_eq!(output, 3);
    }

    #[test]
    fn part_1_prod() {
        let input = include_str!("../input.prod");

        let output = part_1(input);

        assert_eq!(output, 5904);
    }

    #[test]
    fn part_2_dev() {
        let input = include_str!("../input.dev");

        let output = part_2(input);

        assert_eq!(output, 1623178306);
    }

    #[test]
    fn negative_larger_than_len() {
        let mut circular_vec: CircularVec = "1\n2\n-7".parse().unwrap();
        let expected = CircularVec {
            data: VecDeque::from(vec![
                UniqueNumber {
                    value: -7,
                    starting_index: 2,
                },
                UniqueNumber {
                    value: 1,
                    starting_index: 0,
                },
                UniqueNumber {
                    value: 2,
                    starting_index: 1,
                },
            ]),
        };

        circular_vec.swap_all_items(1);

        assert_eq!(circular_vec, expected);
    }

    #[test]
    fn positive_larger_than_len() {
        let mut circular_vec: CircularVec = "1\n2\n7".parse().unwrap();
        let expected = CircularVec {
            data: VecDeque::from(vec![
                UniqueNumber {
                    value: 1,
                    starting_index: 0,
                },
                UniqueNumber {
                    value: 2,
                    starting_index: 1,
                },
                UniqueNumber {
                    value: 7,
                    starting_index: 2,
                },
            ]),
        };

        circular_vec.swap_all_items(1);

        assert_eq!(circular_vec, expected);
    }

    #[test]
    fn duplicates() {
        let mut circular_vec: CircularVec = "1\n2\n3\n2".parse().unwrap();
        let expected = CircularVec {
            data: VecDeque::from(vec![
                UniqueNumber {
                    value: 2,
                    starting_index: 3,
                },
                UniqueNumber {
                    value: 2,
                    starting_index: 1,
                },
                UniqueNumber {
                    value: 1,
                    starting_index: 0,
                },
                UniqueNumber {
                    value: 3,
                    starting_index: 2,
                },
            ]),
        };

        circular_vec.swap_all_items(1);

        assert_eq!(circular_vec, expected);
    }

    #[test]
    fn at_beginning() {
        let input: CircularVec = "1\n2\n3".parse().unwrap();

        let one = input.at(0);

        assert_eq!(one, 1);
    }

    #[test]
    fn at_end() {
        let input: CircularVec = "1\n2\n3".parse().unwrap();

        let three = input.at(2);

        assert_eq!(three, 3);
    }

    #[test]
    fn at_beginning_wrapped() {
        let input: CircularVec = "1\n2\n3".parse().unwrap();

        let one = input.at(3);

        assert_eq!(one, 1);
    }
}
