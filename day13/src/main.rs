use serde_json::Value;
use std::cmp::Ordering;

fn main() {
    println!("Hello, world!");
    let input = include_str!("../input.prod");
    let output = part_2(input);
    println!("output: {output}");
}

#[derive(Debug)]
struct Pair {
    left: Value,
    right: Value,
}

fn value_in_order(left: &Value, right: &Value) -> Ordering {
    println!("Considering in_order on {:?}, {:?}", left, right);
    let mut result: Ordering = Ordering::Equal;
    if let Value::Number(left) = left {
        if let Value::Number(right) = right {
            result = num_in_order(left, right);
        } else if let Value::Array(right_arr) = right {
            let left_arr = vec![Value::Number(left.clone())];
            result = vec_in_order(&left_arr, &right_arr);
        }
    }

    match result {
        Ordering::Less => return Ordering::Less,
        Ordering::Greater => return Ordering::Greater,
        Ordering::Equal => {}
    }

    if let Value::Number(right) = right {
        if let Value::Number(left) = left {
            result = num_in_order(left, right);
        } else if let Value::Array(left_arr) = left {
            let right_arr = vec![Value::Number(right.clone())];
            result = vec_in_order(&left_arr, &right_arr);
        }
    }
    match result {
        Ordering::Less => return Ordering::Less,
        Ordering::Greater => return Ordering::Greater,
        Ordering::Equal => {}
    }

    if let Value::Array(right_arr) = right {
        if let Value::Number(left) = left {
            let left_arr = vec![Value::Number(left.clone())];
            result = vec_in_order(&left_arr, &right_arr);
        } else if let Value::Array(left_arr) = left {
            result = vec_in_order(&left_arr, &right_arr);
        }
    }
    match result {
        Ordering::Less => return Ordering::Less,
        Ordering::Greater => return Ordering::Greater,
        Ordering::Equal => {}
    }

    if let Value::Array(left_arr) = left {
        if let Value::Number(right) = right {
            let right_arr = vec![Value::Number(right.clone())];
            result = vec_in_order(&left_arr, &right_arr);
        } else if let Value::Array(right_arr) = right {
            result = vec_in_order(&left_arr, &right_arr);
        }
    }

    result
}

fn num_in_order(left: &serde_json::Number, right: &serde_json::Number) -> Ordering {
    println!("Considering num_in_order for {:?}, {:?}", left, right);
    let left = left.as_u64();
    let right = right.as_u64();
    left.cmp(&right)
}

fn vec_in_order(left: &Vec<Value>, right: &Vec<Value>) -> Ordering {
    println!("Considering vec_in_order for {:?}, {:?}", left, right);
    let out_of_order_by_length = left.len() <= right.len();
    let temp_vec = vec![left.len(), right.len()];
    let min_length = temp_vec.iter().min().unwrap();
    for i in 0..*min_length {
        let left_val = &left[i];
        let right_val = &right[i];
        let nested_pair = Pair {
            left: left_val.clone(),
            right: right_val.clone(),
        };
        let in_order = value_in_order(left_val, right_val);
        match in_order {
            Ordering::Less => return Ordering::Less,
            Ordering::Greater => return Ordering::Greater,
            Ordering::Equal => {}
        }
    }

    left.len().cmp(&right.len())
}

fn part_1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|double_line| double_line.lines().collect::<Vec<&str>>())
        .map(|line_pair| {
            let left: Value = serde_json::from_str(line_pair[0]).unwrap();
            let right: Value = serde_json::from_str(line_pair[1]).unwrap();
            return Pair { left, right };
        })
        .enumerate()
        .filter(|(_, pair)| {
            let in_order = value_in_order(&pair.left, &pair.right);
            println!("---------------------------\n");
            in_order.is_lt()
        })
        .map(|tuple| tuple.0 + 1)
        .sum()
}

fn part_2(input: &str) -> usize {
    let mut values: Vec<Value> = input
        .split("\n\n")
        .map(|double_line| double_line.lines().collect::<Vec<&str>>())
        .flat_map(|line_pair| {
            let left: Value = serde_json::from_str(line_pair[0]).unwrap();
            let right: Value = serde_json::from_str(line_pair[1]).unwrap();
            return vec![left, right];
        })
        .collect();
    let two: Value = serde_json::from_str("[[2]]").unwrap();
    let six: Value = serde_json::from_str("[[6]]").unwrap();
    values.push(two.clone());
    values.push(six.clone());

    values.sort_by(|left, right| value_in_order(left, right));
    let product = values
        .iter()
        .enumerate()
        .filter_map(|(i, val)| {
            if *val == two || *val == six {
                Some(i + 1)
            } else {
                None
            }
        })
        .product();
    product

    //    values
    //        .iter()
    //        .enumerate()
    //        .filter(|(_, &list)| {
    //            let key_pairs = list == Value::Array(Value::Array(Value::Number(2)))
    //                || list == Value::Array(Value::Array(Value::Number(6)));
    //            println!("---------------------------\n");
    //            in_order.unwrap()
    //        })
    //        .map(|tuple| tuple.0 + 1)
    //        .sum()
}
