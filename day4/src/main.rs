use std::fs;
fn main() {
    println!("Hello, world!");
    let input = fs::read_to_string("./src/input.txt").unwrap();
    let count = part_1(&input);
    println!("Part 1: {:?}", count);
}

struct RangeStruct {
    left: i32,
    right: i32,
}

impl RangeStruct {
    fn from_str(input: &str) -> Option<Self> {
        let mut split_string: Vec<&str> = input.split('-').collect();
        let left = match str::parse::<i32>(split_string[0]) {
            Ok(value) => value,
            Err(_) => return None,
        };
        let right = match str::parse::<i32>(split_string[1]) {
            Ok(value) => value,
            Err(_) => return None,
        };
        return Some(RangeStruct { left, right });
    }
}

type Range = (i32, i32);

fn split_range(string: &str) -> Range {
    let mut split_string: Vec<&str> = string.split('-').collect();
    let second_num = str::parse::<i32>(split_string.pop().unwrap()).unwrap();
    let first_num = str::parse::<i32>(split_string.pop().unwrap()).unwrap();
    return (first_num, second_num);
}

fn part_1(input: &str) -> u32 {
    let pairs: Vec<&str> = input.split("\n").collect();
    let count: u32 = pairs
        .iter()
        .filter(|&&pair| !pair.eq(""))
        .map(|pair| {
            let split: Vec<&str> = pair.split(",").collect();
            let first = RangeStruct::from_str(split[0]).unwrap();
            let second = RangeStruct::from_str(split[1]).unwrap();
            if (first.right >= second.right && first.left <= second.left) {
                return 1;
            } else if (second.right >= first.right && second.left <= first.left) {
                return 1;
            } else {
                return 0;
            }
        })
        .sum();

    return count;
}

fn part_2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let pairs: Vec<&str> = input.split("\n").collect();
    let count: i32 = pairs
        .iter()
        .filter(|&&pair| !pair.eq(""))
        .map(|pair| {
            let mut split: Vec<&str> = pair.split(",").collect();
            let first = split_range(split.pop().unwrap());
            let second = split_range(split.pop().unwrap());
            let first_overlap = pair_overlap(&first, &second);
            if (first_overlap == 1) {
                return first_overlap;
            }
            let second_overlap = pair_overlap(&second, &first);
            return second_overlap;
        })
        .sum();

    println!("Part 2: {:?}", count);

    //    println!("Part 1: {:?}");
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::part_1;

    #[test]
    fn aoc_solution() {
        let input = fs::read_to_string("./src/input.txt").unwrap();

        let count = part_1(&input);

        assert_eq!(count, 441);
    }
}

fn pair_overlap(first: &Range, second: &Range) -> i32 {
    // lower end of first overlaps
    if (first.0 >= second.0 && first.0 <= second.1) {
        return 1;
    };
    // upper end of first overlaps
    if (first.1 <= second.1 && first.1 >= second.0) {
        return 1;
    };
    return 0;
}
