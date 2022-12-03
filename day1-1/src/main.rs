use std::fs;
fn main() {
    part_1();
    part_2();
}

fn part_2() {
    let file = fs::read_to_string("input.txt").unwrap();
    let elf_inventory_string = file.split("\n\n");
    let mut elf_inventories: Vec<i32> = elf_inventory_string
        .map(|elf| {
            let total_calories: i32 = elf
                .split("\n")
                .map(|calorie_string| str::parse::<i32>(calorie_string).unwrap_or(0))
                .sum();
            total_calories
        })
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
    let elf_inventories = elf_inventory_string.map(|elf| {
        let total_calories: i32 = elf
            .split("\n")
            .map(|calorie_string| str::parse::<i32>(calorie_string).unwrap_or(0))
            .sum();
        total_calories
    });
    let max_elf_inventory = elf_inventories.max();
    println!("Max inventory is {:?}", max_elf_inventory);
}
