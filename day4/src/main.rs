use std::fs;
fn main() {
    println!("Hello, world!");
    part_1();
    part_2();
}

fn part_1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let pairs: Vec<&str> = input.split("\n").collect();
    let count: i32 = pairs
        .iter()
        .filter(|&&pair| !pair.eq(""))
        .map(|pair| {
            let mut split: Vec<&str> = pair.split(",").collect();
            let first = split_range(split.pop().unwrap());
            let second = split_range(split.pop().unwrap());
            if (first.1 >= second.1 && first.0 <= second.0) {
                return 1;
            } else if (second.1 >= first.1 && second.0 <= first.0) {
                return 1;
            } else {
                return 0;
            }
        })
        .sum();

    println!("Part 1: {:?}", count);
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

type Range = (i32, i32);

fn split_range(string: &str) -> Range {
    let mut split_string: Vec<&str> = string.split('-').collect();
    let second_num = str::parse::<i32>(split_string.pop().unwrap()).unwrap();
    let first_num = str::parse::<i32>(split_string.pop().unwrap()).unwrap();
    return (first_num, second_num);
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
