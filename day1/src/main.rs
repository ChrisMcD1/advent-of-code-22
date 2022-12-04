use std::fs;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    part_1();
    let part_1_elapsed = now.elapsed();
    println!("Part 1 Elapsed: {:.2?}", part_1_elapsed);
    part_2();
    let part_2_elapsed = now.elapsed();
    println!("Part 2 Elapsed: {:.2?}", part_2_elapsed - part_1_elapsed);
}

fn part_2() {
    let file = fs::read_to_string("input.txt").unwrap();
    let elf_inventory_string = file.split("\n\n");
    let mut elf_inventories: Vec<i32> = elf_inventory_string
        .map(|elf| calculate_elf_calories(elf))
        .collect();
    elf_inventories.sort();
    let max_elf_inventory: i32 = elf_inventories[elf_inventories.len() - 3..]
        .into_iter()
        .sum();
    println!("Sum of 3 max inventory is {:?}", max_elf_inventory);
}

fn part_1() {
    let file = fs::read_to_string("input.txt").unwrap();
    let elf_inventory_string = file.split("\n\n");
    let elf_inventories = elf_inventory_string.map(|elf| calculate_elf_calories(elf));
    let max_elf_inventory = elf_inventories.max();
    println!("Max inventory is {:?}", max_elf_inventory);
}

fn calculate_elf_calories(elf: &str) -> i32 {
    elf.split("\n")
        .map(|calorie_string| str::parse::<i32>(calorie_string).unwrap_or(0))
        .sum()
}

#[cfg(test)]
mod test {
    use crate::calculate_elf_calories;

    #[test]
    fn elf_calories_basic() {
        let str = "1\n1\n1\n1";

        let sum = calculate_elf_calories(str);

        assert_eq!(sum, 4);
    }
    #[test]
    fn elf_calories_trailing_newline() {
        let str = "1\n1\n1\n1\n";

        let sum = calculate_elf_calories(str);

        assert_eq!(sum, 4);
    }
}
