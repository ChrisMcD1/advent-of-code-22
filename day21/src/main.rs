use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

fn main() {
    println!("Hello, world!");
    let input = include_str!("../input.prod");
    let output = part_1(input);
    println!("Monkey yells {output}");
    let output = part_2(input);
    println!("Human should yell {output}");
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct MathEquation {
    left: String,
    right: String,
    operator: Operator,
}

impl MathEquation {
    fn new(left: String, right: String, operator: Operator) -> Self {
        return Self {
            left,
            right,
            operator,
        };
    }
    fn calculate_operator(&self, left_value: i64, right_value: i64) -> i64 {
        return match self.operator {
            Operator::Add => left_value + right_value,
            Operator::Subtract => left_value - right_value,
            Operator::Multiply => left_value * right_value,
            Operator::Divide => left_value / right_value,
        };
    }
    fn get_value(&self, all_monkeys: &HashMap<String, Monkey>) -> i64 {
        let left_value = all_monkeys.get(&self.left).unwrap().get_value(all_monkeys);
        let right_value = all_monkeys.get(&self.right).unwrap().get_value(all_monkeys);
        return self.calculate_operator(left_value, right_value);
    }
    fn get_optional_value(&self, all_monkeys: &HashMap<String, Monkey>) -> Option<i64> {
        let left_value = all_monkeys
            .get(&self.left)
            .unwrap()
            .get_optional_value(all_monkeys)?;
        let right_value = all_monkeys
            .get(&self.right)
            .unwrap()
            .get_optional_value(all_monkeys)?;
        return Some(self.calculate_operator(left_value, right_value));
    }
    fn find_value_to_make_equal(
        &self,
        target_value: i64,
        all_monkeys: &HashMap<String, Monkey>,
    ) -> i64 {
        let left_monkey_value = all_monkeys
            .get(&self.left)
            .unwrap()
            .get_optional_value(all_monkeys);
        let right_monkey_value = all_monkeys
            .get(&self.right)
            .unwrap()
            .get_optional_value(all_monkeys);

        if let Some(left_concrete) = left_monkey_value {
            let new_target = match self.operator {
                Operator::Add => target_value - left_concrete,
                Operator::Subtract => -1 * (target_value - left_concrete),
                Operator::Multiply => target_value / left_concrete,
                Operator::Divide => left_concrete / target_value,
            };
            return all_monkeys
                .get(&self.right)
                .unwrap()
                .find_value_to_make_equal(new_target, all_monkeys);
        } else if let Some(right_concrete) = right_monkey_value {
            let new_target = match self.operator {
                Operator::Add => target_value - right_concrete,
                Operator::Subtract => target_value + right_concrete,
                Operator::Multiply => target_value / right_concrete,
                Operator::Divide => target_value * right_concrete,
            };
            return all_monkeys
                .get(&self.left)
                .unwrap()
                .find_value_to_make_equal(new_target, all_monkeys);
        } else {
            unreachable!()
        }
    }
}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
enum MonkeyAlgorithm {
    Equation(MathEquation),
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
            MonkeyAlgorithm::Equation(math_equation) => math_equation.get_value(all_monkeys),
        };
    }
    fn get_optional_value(&self, all_monkeys: &HashMap<String, Monkey>) -> Option<i64> {
        if self.name == "humn" {
            return None;
        }
        return match &self.algorithm {
            MonkeyAlgorithm::Constant(val) => Some(*val),
            MonkeyAlgorithm::Equation(math_equation) => {
                math_equation.get_optional_value(all_monkeys)
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
            MonkeyAlgorithm::Equation(math_equation) => {
                math_equation.find_value_to_make_equal(target_value, all_monkeys)
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
            let operator = match operation {
                "+" => Operator::Add,
                "-" => Operator::Subtract,
                "/" => Operator::Divide,
                "*" => Operator::Multiply,
                _ => unreachable!(),
            };
            return Ok(MonkeyAlgorithm::Equation(MathEquation::new(
                left_monkey.to_string(),
                right_monkey.to_string(),
                operator,
            )));
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
    if let MonkeyAlgorithm::Equation(math_equation) = &root_monkey.algorithm {
        let left_monkey = monkeys.get(&math_equation.left).unwrap();
        let right_monkey = monkeys.get(&math_equation.right).unwrap();
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
            algorithm: MonkeyAlgorithm::Equation(MathEquation::new(
                "humn".to_string(),
                "aaaa".to_string(),
                Operator::Add,
            )),
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
            algorithm: MonkeyAlgorithm::Equation(MathEquation::new(
                "humn".to_string(),
                "aaaa".to_string(),
                Operator::Subtract,
            )),
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
            algorithm: MonkeyAlgorithm::Equation(MathEquation::new(
                "humn".to_string(),
                "aaaa".to_string(),
                Operator::Multiply,
            )),
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
            algorithm: MonkeyAlgorithm::Equation(MathEquation::new(
                "humn".to_string(),
                "aaaa".to_string(),
                Operator::Divide,
            )),
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
}
