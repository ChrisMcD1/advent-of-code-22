use itertools::Itertools;
use std::time::Instant;
use std::{fs, str::FromStr};

fn main() {
    println!("Hello, world!");
    let now = Instant::now();
    part_1();
    let part_1_elapsed = now.elapsed();
    println!("Part 1 Elapsed: {:.2?}", part_1_elapsed);
    part_2();
    let part_2_elapsed = now.elapsed();
    println!("Part 2 Elapsed: {:.2?}", part_2_elapsed);
}

fn part_1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let rucksacks_str = input.split("\n");
    let rucksacks: Vec<Rucksack> = rucksacks_str
        .map(|sack| Rucksack::from_str(sack))
        .flatten()
        .collect();
    let total_sum: u32 = rucksacks
        .into_iter()
        .map(|sack| sack.find_shared_item().expect("Cannot Find shared item!"))
        .map(|item| get_priority_for_item(item))
        .sum();
    println!("Found total priority of {:?}", total_sum);
}

fn part_2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let rucksacks_str = input.split("\n");
    let rucksacks: Vec<Rucksack> = rucksacks_str
        .map(|sack| Rucksack::from_str(sack))
        .flatten()
        .collect();
    let groups = rucksacks
        .into_iter()
        .tuples()
        .map(|(sack1, sack2, sack3)| Group {
            sack1,
            sack2,
            sack3,
        });
    //    println!("groups: {:?}", groups);
    let total_sum: u32 = groups
        .into_iter()
        .map(|group| group.find_shared_item().expect("Cannot Find shared item!"))
        .map(|item| get_priority_for_item(item))
        .sum();
    println!("Found total priority of groups of {:?}", total_sum);
}

#[derive(Debug)]
struct Group {
    sack1: Rucksack,
    sack2: Rucksack,
    sack3: Rucksack,
}

impl Group {
    fn find_shared_item(self) -> Option<Item> {
        let mut total_shared: Option<Item> = None;
        for item in self.sack1.full_contents {
            let item_in_2 = self.sack2.full_contents.contains(&item);
            if item_in_2 {
                let item_in_3 = self.sack3.full_contents.contains(&item);
                if item_in_3 {
                    total_shared = Some(item);
                    break;
                }
            }
        }

        total_shared
    }
}

#[derive(Debug)]
struct Rucksack {
    full_contents: Vec<Item>,
    left: Vec<Item>,
    right: Vec<Item>,
}

type Item = char;

impl Rucksack {
    fn from_str(input: &str) -> Option<Self> {
        let input = String::from_str(input).unwrap();
        let rucksack_len = input.len() / 2;
        let left: Vec<Item> = input.clone()[..rucksack_len].chars().collect();
        let right: Vec<Item> = input.clone()[rucksack_len..].chars().collect();
        if left.len() == 0 || right.len() == 0 {
            return None;
        };
        let mut full_contents = left.clone();
        full_contents.append(&mut right.clone());
        Some(Rucksack {
            full_contents,
            left,
            right,
        })
    }
    fn find_shared_item(self) -> Option<Item> {
        let mut shared_item = None;
        for item in self.left {
            let item_in_right = self.right.contains(&item);
            if item_in_right {
                shared_item = Some(item.clone());
                break;
            }
        }
        shared_item
    }
}

fn get_priority_for_item(item: Item) -> u32 {
    let num = item as u32;
    if num > 64 && num < 91 {
        num - 38
    } else {
        num - 96
    }
}
