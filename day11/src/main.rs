use anyhow::{anyhow, Result};
use std::cell::RefCell;
use std::rc::Rc;
use std::{collections::VecDeque, str::FromStr};

fn main() {
    //println!("Hello, world!");
    let input = include_str!("real_1");
    let result = part_1(input).unwrap();
    println!("Result: {result}");
}

#[derive(Debug)]
enum NumOrOld {
    Num(i64),
    Old,
}

impl FromStr for NumOrOld {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "old" => NumOrOld::Old,
            _ => NumOrOld::Num(s.parse().unwrap()),
        })
    }
}

#[derive(Debug)]
enum Operation {
    Add(NumOrOld),
    Multiply(NumOrOld),
}

impl FromStr for Operation {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let key_part = s.split_once("=").unwrap().1.trim();

        let operand = key_part.chars().clone().nth(4).unwrap();

        let mut buffer = [0; 1];
        let operand_as_str = operand.encode_utf8(&mut buffer);

        let num_or_old: NumOrOld = key_part.split(|c| c == operand).collect::<Vec<&str>>()[1]
            .trim()
            .parse()
            .unwrap();

        if operand_as_str == "+" {
            return Ok(Self::Add(num_or_old));
        } else if operand_as_str == "*" {
            return Ok(Self::Multiply(num_or_old));
        } else {
            return Err(anyhow!("bod"));
        }
    }
}

#[derive(Debug)]
struct Test {
    divisible_by: i64,
    true_monkey: usize,
    false_monkey: usize,
}

impl FromStr for Test {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        let test_line = lines[0];
        let true_line = lines[1];
        let false_line = lines[2];
        let divisible_by: i64 = test_line
            .split_whitespace()
            .collect::<Vec<&str>>()
            .pop()
            .unwrap()
            .parse()
            .unwrap();
        let true_monkey: usize = true_line
            .split_whitespace()
            .collect::<Vec<&str>>()
            .pop()
            .unwrap()
            .parse()
            .unwrap();
        let false_monkey: usize = false_line
            .split_whitespace()
            .collect::<Vec<&str>>()
            .pop()
            .unwrap()
            .parse()
            .unwrap();

        return Ok(Self {
            divisible_by,
            true_monkey,
            false_monkey,
        });
    }
}

impl Test {
    fn find_next_monkey(&self, value: &i64) -> usize {
        if value % self.divisible_by == 0 {
            return self.true_monkey;
        } else {
            return self.false_monkey;
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<i64>,
    operation: Operation,
    test: Test,
    inspected_items: u64,
}

impl FromStr for Monkey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        let starting_items = lines[1];
        let operation = lines[2];
        let test = lines[3..=5]
            .iter()
            .map(|str| str.to_string())
            .reduce(|mut accum, item| {
                accum = accum + "\n" + &item;
                return accum;
            })
            .unwrap();
        let starting_items: VecDeque<i64> = starting_items
            .split_once(":")
            .unwrap()
            .1
            .split(",")
            .map(|elem| {
                return elem.trim().parse::<i64>().unwrap();
            })
            .collect();
        let operation: Operation = operation.parse().unwrap();
        let test: Test = test.parse().unwrap();
        return Ok(Self {
            items: starting_items,
            operation,
            test,
            inspected_items: 0,
        });
    }
}

impl Monkey {
    fn inspect_items(&mut self, other_monkeys: &Vec<Rc<RefCell<Monkey>>>, lcm: i64) {
        while !self.items.is_empty() {
            let item = self.items.pop_front().unwrap();
            //println!("Monkey Inspects an item with a worry level of {item}");
            let higher_stress: i64 = match &self.operation {
                Operation::Add(num_or_old) => {
                    item + match num_or_old {
                        NumOrOld::Old => item,
                        NumOrOld::Num(other_num) => *other_num,
                    }
                }
                Operation::Multiply(num_or_old) => {
                    item * match num_or_old {
                        NumOrOld::Old => item,
                        NumOrOld::Num(other_num) => *other_num,
                    }
                }
            };
            //          println!("Worry level is increased to {higher_stress}");
            let new_stress = higher_stress % lcm;
            //           println!("New stress is {new_stress}");
            let test_result = self.test.find_next_monkey(&new_stress);
            //            println!("Item of stress {new_stress} is being passed to monkey {test_result}");
            let mut target_monkey = other_monkeys[test_result].borrow_mut();
            target_monkey.items.push_back(new_stress);
            self.inspected_items = self.inspected_items + 1;
        }
    }
}

fn part_1(input: &str) -> Result<i64> {
    let monkeys_str = input.split("\n\n");
    let mut monkeys: Vec<Rc<RefCell<Monkey>>> = monkeys_str
        .map(|str| str.parse().unwrap())
        .map(|monkey| Rc::new(RefCell::new(monkey)))
        .collect();
    println!("Got monkeys of {:?}", monkeys);

    let lcm = monkeys
        .iter()
        .map(|monkey| monkey.borrow().test.divisible_by)
        .product();
    println!("FOund base lcm of {lcm}");
    for _ in 0..10000 {
        for monkey in &monkeys {
            monkey.borrow_mut().inspect_items(&monkeys, lcm);
        }
    }
    monkeys.sort_by(|left, right| {
        left.borrow()
            .inspected_items
            .cmp(&right.borrow().inspected_items)
    });

    let top_monkey = monkeys.pop().unwrap();
    let second_top_monkey = monkeys.pop().unwrap();
    println!(
        "First mokey inspected, {:?}, and second monkey: {:?}",
        top_monkey.borrow().inspected_items,
        second_top_monkey.borrow().inspected_items
    );

    let result = top_monkey.borrow().inspected_items * second_top_monkey.borrow().inspected_items;

    return Ok(result as i64);
}
