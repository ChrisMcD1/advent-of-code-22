use anyhow::Result;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn find_unique_of_size(input: &str, length: usize) -> Result<usize> {
    let max = input.len();
    let mut buffer = VecDeque::new();
    let mut input_chars: VecDeque<char> = input.chars().collect();
    for _ in 0..length - 1 {
        buffer.push_back(input_chars.pop_front().unwrap());
    }
    for index in length - 1..max {
        buffer.push_back(input_chars.pop_front().unwrap());
        let unique = fully_unique(&buffer);
        if unique {
            return Ok(index + 1);
        }
        buffer.pop_front();
    }
    unreachable!("Could not find a unique set");
}

pub fn find_unique_of_size_map(input: &str, length: usize) -> Result<usize> {
    let max = input.len();
    let mut buffer = SmartBuffer::new();
    let mut input_chars: VecDeque<char> = input.chars().collect();
    for _ in 0..length - 1 {
        let new_char = input_chars.pop_front().unwrap();
        buffer.add(new_char);
    }
    for index in length - 1..max {
        let new_char = input_chars.pop_front().unwrap();
        buffer.add(new_char);
        //println!("{:?}", buffer);

        let unique = buffer.only_has_unique();
        if unique {
            return Ok(index + 1);
        }

        buffer.pop();
    }
    unreachable!("Could not find a unique set");
}

#[derive(Debug)]
pub struct SmartBuffer {
    vec: VecDeque<char>,
    main_map: HashMap<char, usize>,
    hot_map: HashMap<char, usize>,
    hot_map_threshold: usize,
}

impl SmartBuffer {
    pub fn new() -> Self {
        Self {
            vec: VecDeque::new(),
            main_map: HashMap::new(),
            hot_map: HashMap::new(),
            hot_map_threshold: 2,
        }
    }
    pub fn add(self: &mut Self, new_char: char) {
        let current_count: usize = match self.main_map.get(&new_char) {
            Some(num) => *num,
            _ => 0,
        };
        let new_count = current_count + 1;
        if new_count > self.hot_map_threshold {
            self.hot_map.insert(new_char, new_count);
        }
        self.main_map.insert(new_char, new_count);
        self.vec.push_back(new_char);
    }
    pub fn pop(self: &mut Self) {
        let old_char = self.vec.pop_front().unwrap();
        let current_count: usize = match self.main_map.get(&old_char) {
            Some(num) => *num,
            _ => unreachable!(),
        };
        let new_count = current_count - 1;
        if new_count == self.hot_map_threshold {
            self.hot_map.remove(&old_char);
        }
        if new_count == 0 {
            self.main_map.remove(&old_char);
        } else {
            self.main_map.insert(old_char, new_count);
        }
    }
    pub fn only_has_unique(&self) -> bool {
        let hot_empty = self.hot_map.is_empty();
        return hot_empty && self.main_map.iter().all(|(_, &value)| value == 1);
    }
}

pub fn fully_unique(vec: &VecDeque<char>) -> bool {
    let mut uniq = HashSet::new();
    vec.clone().into_iter().all(move |x| uniq.insert(x))
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn part_1_no_map() {
        let input = include_str!("./given");

        let output = find_unique_of_size(input, 4).unwrap();

        assert_eq!(output, 7);
    }

    #[test]
    fn part_1_map() {
        let input = include_str!("./given");

        let output = find_unique_of_size_map(input, 4).unwrap();

        assert_eq!(output, 7);
    }

    #[test]
    fn part_2_map() {
        let input = include_str!("./real");

        let output = find_unique_of_size_map(input, 14).unwrap();

        assert_eq!(output, 3298);
    }

    #[test]
    fn part_2_no_map() {
        let input = include_str!("./real");

        let output = find_unique_of_size(input, 14).unwrap();

        assert_eq!(output, 3298);
    }
}
