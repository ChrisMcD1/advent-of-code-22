use std::{collections::HashMap, str::FromStr};

fn main() {
    println!("Hello, world!");
    let input = include_str!("../input.prod");
    //    let output = part_1(input);
    //   println!("Monkey yells {output}");
    let output = part_2(input);
    println!("Human should yell {output}");
}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
enum MonkeyAlgorithm {
    Add(String, String),
    Multiply(String, String),
    Divide(String, String),
    Subtract(String, String),
    Constant(i64),
}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
struct Monkey {
    name: String,
    algorithm: MonkeyAlgorithm,
}

impl FromStr for Monkey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, algorithm) = s.split_once(":").unwrap();
        return Ok(Monkey {
            name: name.to_string(),
            algorithm: algorithm.parse().unwrap(),
        });
    }
}

impl Monkey {
    fn get_value(&self, all_monkeys: &HashMap<String, Monkey>) -> i64 {
        return match &self.algorithm {
            MonkeyAlgorithm::Constant(val) => *val,
            MonkeyAlgorithm::Add(left, right) => {
                all_monkeys.get(left).unwrap().get_value(all_monkeys)
                    + all_monkeys.get(right).unwrap().get_value(all_monkeys)
            }
            MonkeyAlgorithm::Multiply(left, right) => {
                all_monkeys.get(left).unwrap().get_value(all_monkeys)
                    * all_monkeys.get(right).unwrap().get_value(all_monkeys)
            }
            MonkeyAlgorithm::Divide(left, right) => {
                all_monkeys.get(left).unwrap().get_value(all_monkeys)
                    / all_monkeys.get(right).unwrap().get_value(all_monkeys)
            }
            MonkeyAlgorithm::Subtract(left, right) => {
                all_monkeys.get(left).unwrap().get_value(all_monkeys)
                    - all_monkeys.get(right).unwrap().get_value(all_monkeys)
            }
        };
    }
    fn get_optional_value(&self, all_monkeys: &HashMap<String, Monkey>) -> Option<i64> {
        if self.name == "humn" {
            return None;
        }
        return match &self.algorithm {
            MonkeyAlgorithm::Constant(val) => Some(*val),
            MonkeyAlgorithm::Add(left, right) => {
                let left_monkey_value = all_monkeys
                    .get(left)
                    .unwrap()
                    .get_optional_value(all_monkeys)?;
                let right_monkey_value = all_monkeys
                    .get(right)
                    .unwrap()
                    .get_optional_value(all_monkeys)?;
                Some(left_monkey_value + right_monkey_value)
            }
            MonkeyAlgorithm::Subtract(left, right) => {
                let left_monkey_value = all_monkeys
                    .get(left)
                    .unwrap()
                    .get_optional_value(all_monkeys)?;
                let right_monkey_value = all_monkeys
                    .get(right)
                    .unwrap()
                    .get_optional_value(all_monkeys)?;
                Some(left_monkey_value - right_monkey_value)
            }
            MonkeyAlgorithm::Multiply(left, right) => {
                let left_monkey_value = all_monkeys
                    .get(left)
                    .unwrap()
                    .get_optional_value(all_monkeys)?;
                let right_monkey_value = all_monkeys
                    .get(right)
                    .unwrap()
                    .get_optional_value(all_monkeys)?;
                Some(left_monkey_value * right_monkey_value)
            }
            MonkeyAlgorithm::Divide(left, right) => {
                let left_monkey_value = all_monkeys
                    .get(left)
                    .unwrap()
                    .get_optional_value(all_monkeys)?;
                let right_monkey_value = all_monkeys
                    .get(right)
                    .unwrap()
                    .get_optional_value(all_monkeys)?;
                Some(left_monkey_value / right_monkey_value)
            }
        };
    }
    fn find_value_to_make_equal(
        &self,
        target_value: i64,
        all_monkeys: &HashMap<String, Monkey>,
    ) -> i64 {
        if self.name == "humn".to_string() {
            return target_value;
        }
        match &self.algorithm {
            MonkeyAlgorithm::Add(left, right) => {
                let left_monkey_value = all_monkeys
                    .get(left)
                    .unwrap()
                    .get_optional_value(all_monkeys);
                let right_monkey_value = all_monkeys
                    .get(right)
                    .unwrap()
                    .get_optional_value(all_monkeys);

                if let Some(left_concrete) = left_monkey_value {
                    let new_target = target_value - left_concrete;
                    return all_monkeys
                        .get(right)
                        .unwrap()
                        .find_value_to_make_equal(new_target, all_monkeys);
                } else if let Some(right_concrete) = right_monkey_value {
                    let new_target = target_value - right_concrete;
                    return all_monkeys
                        .get(left)
                        .unwrap()
                        .find_value_to_make_equal(new_target, all_monkeys);
                } else {
                    unreachable!()
                }
            }
            MonkeyAlgorithm::Subtract(left, right) => {
                let left_monkey_value = all_monkeys
                    .get(left)
                    .unwrap()
                    .get_optional_value(all_monkeys);
                let right_monkey_value = all_monkeys
                    .get(right)
                    .unwrap()
                    .get_optional_value(all_monkeys);

                if let Some(left_concrete) = left_monkey_value {
                    let new_target = -1 * (target_value - left_concrete);
                    return all_monkeys
                        .get(right)
                        .unwrap()
                        .find_value_to_make_equal(new_target, all_monkeys);
                } else if let Some(right_concrete) = right_monkey_value {
                    let new_target = target_value + right_concrete;
                    return all_monkeys
                        .get(left)
                        .unwrap()
                        .find_value_to_make_equal(new_target, all_monkeys);
                } else {
                    unreachable!()
                }
            }
            MonkeyAlgorithm::Multiply(left, right) => {
                let left_monkey_value = all_monkeys
                    .get(left)
                    .unwrap()
                    .get_optional_value(all_monkeys);
                let right_monkey_value = all_monkeys
                    .get(right)
                    .unwrap()
                    .get_optional_value(all_monkeys);

                if let Some(left_concrete) = left_monkey_value {
                    let new_target = target_value / left_concrete;
                    return all_monkeys
                        .get(right)
                        .unwrap()
                        .find_value_to_make_equal(new_target, all_monkeys);
                } else if let Some(right_concrete) = right_monkey_value {
                    let new_target = target_value / right_concrete;
                    return all_monkeys
                        .get(left)
                        .unwrap()
                        .find_value_to_make_equal(new_target, all_monkeys);
                } else {
                    unreachable!()
                }
            }
            MonkeyAlgorithm::Divide(left, right) => {
                let left_monkey_value = all_monkeys
                    .get(left)
                    .unwrap()
                    .get_optional_value(all_monkeys);
                let right_monkey_value = all_monkeys
                    .get(right)
                    .unwrap()
                    .get_optional_value(all_monkeys);

                if let Some(left_concrete) = left_monkey_value {
                    let new_target = left_concrete / target_value;
                    return all_monkeys
                        .get(right)
                        .unwrap()
                        .find_value_to_make_equal(new_target, all_monkeys);
                } else if let Some(right_concrete) = right_monkey_value {
                    let new_target = target_value * right_concrete;
                    return all_monkeys
                        .get(left)
                        .unwrap()
                        .find_value_to_make_equal(new_target, all_monkeys);
                } else {
                    unreachable!()
                }
            }
            MonkeyAlgorithm::Constant(val) => *val,
        }
    }
}

impl FromStr for MonkeyAlgorithm {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = s.trim().split_whitespace().collect();
        if words.len() == 1 {
            return Ok(MonkeyAlgorithm::Constant(words[0].parse().unwrap()));
        } else if words.len() == 3 {
            let left_monkey = words[0];
            let operation = words[1];
            let right_monkey = words[2];
            match operation {
                "+" => {
                    return Ok(MonkeyAlgorithm::Add(
                        left_monkey.to_string(),
                        right_monkey.to_string(),
                    ))
                }
                "-" => {
                    return Ok(MonkeyAlgorithm::Subtract(
                        left_monkey.to_string(),
                        right_monkey.to_string(),
                    ))
                }
                "*" => {
                    return Ok(MonkeyAlgorithm::Multiply(
                        left_monkey.to_string(),
                        right_monkey.to_string(),
                    ))
                }
                "/" => {
                    return Ok(MonkeyAlgorithm::Divide(
                        left_monkey.to_string(),
                        right_monkey.to_string(),
                    ))
                }
                _ => unreachable!(),
            }
        } else {
            unreachable!()
        }
    }
}

fn part_1(input: &str) -> i64 {
    let monkeys: HashMap<String, Monkey> = input
        .lines()
        .map(|line| {
            let monkey = line.parse::<Monkey>().unwrap();
            return (monkey.name.clone(), monkey);
        })
        .collect();
    let root_monkey = monkeys
        .iter()
        .find(|&elem| elem.1.name == "root".to_string())
        .unwrap()
        .1;
    return root_monkey.get_value(&monkeys);
}

fn part_2(input: &str) -> i64 {
    let monkeys: HashMap<String, Monkey> = input
        .lines()
        .map(|line| {
            let monkey = line.parse::<Monkey>().unwrap();
            return (monkey.name.clone(), monkey);
        })
        .collect();
    let root_monkey = monkeys
        .iter()
        .find(|&elem| elem.1.name == "root".to_string())
        .unwrap()
        .1;
    if let MonkeyAlgorithm::Add(left, right) = &root_monkey.algorithm {
        let left_monkey = monkeys.get(left).unwrap();
        let right_monkey = monkeys.get(right).unwrap();
        let left_monkey_value = left_monkey.get_optional_value(&monkeys);
        let right_monkey_value = right_monkey.get_optional_value(&monkeys);
        if let Some(left_concrete) = left_monkey_value {
            println!("Left is concretely: {left_concrete}");
            return right_monkey.find_value_to_make_equal(left_concrete, &monkeys);
        } else if let Some(right_concrete) = right_monkey_value {
            println!("Right is concretely: {right_concrete}");
            return left_monkey.find_value_to_make_equal(right_concrete, &monkeys);
        } else {
            unreachable!()
        }
    } else {
        unreachable!()
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn part_1_given() {
        let input = include_str!("../input.dev");

        let output = part_1(input);

        assert_eq!(output, 152);
    }
    #[test]
    fn part_2_given() {
        let input = include_str!("../input.dev");

        let output = part_2(input);

        assert_eq!(output, 301);
    }

    fn get_human() -> Monkey {
        Monkey {
            name: "humn".to_string(),
            algorithm: MonkeyAlgorithm::Constant(0),
        }
    }

    #[test]
    fn add_equal() {
        let left = get_human();
        let right = Monkey {
            name: "aaaa".to_string(),
            algorithm: MonkeyAlgorithm::Constant(10),
        };
        let root = Monkey {
            name: "root".to_string(),
            algorithm: MonkeyAlgorithm::Add("humn".to_string(), "aaaa".to_string()),
        };
        let all_monkeys: HashMap<String, Monkey> = vec![
            (left.name.clone(), left),
            (right.name.clone(), right),
            (root.name.clone(), root.clone()),
        ]
        .into_iter()
        .collect();

        let equal_value = root.find_value_to_make_equal(100, &all_monkeys);

        assert_eq!(equal_value, 90);
    }

    #[test]
    fn subtract() {
        let left = get_human();
        let right = Monkey {
            name: "aaaa".to_string(),
            algorithm: MonkeyAlgorithm::Constant(10),
        };
        let root = Monkey {
            name: "root".to_string(),
            algorithm: MonkeyAlgorithm::Subtract("humn".to_string(), "aaaa".to_string()),
        };
        let all_monkeys: HashMap<String, Monkey> = vec![
            (left.name.clone(), left),
            (right.name.clone(), right),
            (root.name.clone(), root.clone()),
        ]
        .into_iter()
        .collect();

        let equal_value = root.find_value_to_make_equal(100, &all_monkeys);

        assert_eq!(equal_value, 110);
    }
    #[test]
    fn multiply_equal() {
        let left = get_human();
        let right = Monkey {
            name: "aaaa".to_string(),
            algorithm: MonkeyAlgorithm::Constant(10),
        };
        let root = Monkey {
            name: "root".to_string(),
            algorithm: MonkeyAlgorithm::Multiply("humn".to_string(), "aaaa".to_string()),
        };
        let all_monkeys: HashMap<String, Monkey> = vec![
            (left.name.clone(), left),
            (right.name.clone(), right),
            (root.name.clone(), root.clone()),
        ]
        .into_iter()
        .collect();

        let equal_value = root.find_value_to_make_equal(100, &all_monkeys);

        assert_eq!(equal_value, 10);
    }
    #[test]
    fn divide_equal() {
        let left = get_human();
        let right = Monkey {
            name: "aaaa".to_string(),
            algorithm: MonkeyAlgorithm::Constant(10),
        };
        let root = Monkey {
            name: "root".to_string(),
            algorithm: MonkeyAlgorithm::Divide("humn".to_string(), "aaaa".to_string()),
        };
        let all_monkeys: HashMap<String, Monkey> = vec![
            (left.name.clone(), left),
            (right.name.clone(), right),
            (root.name.clone(), root.clone()),
        ]
        .into_iter()
        .collect();

        let equal_value = root.find_value_to_make_equal(100, &all_monkeys);

        assert_eq!(equal_value, 1000);
    }

    #[test]
    fn monkey_constant_parse() {
        let input = "abcd: 100";

        let parsed_monkey: Monkey = input.parse().unwrap();

        assert_eq!(
            parsed_monkey,
            Monkey {
                name: "abcd".to_string(),
                algorithm: MonkeyAlgorithm::Constant(100)
            }
        )
    }

    #[test]
    fn monkey_add_parse() {
        let input = "abcd: aaaa + bbbb";

        let parsed_monkey: Monkey = input.parse().unwrap();

        assert_eq!(
            parsed_monkey,
            Monkey {
                name: "abcd".to_string(),
                algorithm: MonkeyAlgorithm::Add("aaaa".to_string(), "bbbb".to_string())
            }
        )
    }
}
