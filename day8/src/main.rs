use std::time::Instant;

use anyhow::Result;
fn main() {
    println!("Hello, world!");
    let real_input = include_str!("./1_real");

    let tree_count = part_1(real_input).unwrap();

    println!("There are {tree_count} trees");
    let start = Instant::now();
    let best_score = part_2(real_input);
    let end = Instant::now();
    let elapsed = end - start;
    println!("Ran pant 2 in {:?}", elapsed);
    println!("The best tree house has a score of {best_score}");
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Tree {
    height: u32,
}

type Forest = Vec<Vec<Tree>>;

fn is_tree_visible(forest: &Forest, x: usize, y: usize) -> bool {
    let target_tree = forest[x][y];

    let tree_slices = TreeVisiblitySlices::create_for_tree(forest, x, y);

    return tree_visible_in_slice(&tree_slices.left, &target_tree)
        || tree_visible_in_slice(&tree_slices.right, &target_tree)
        || tree_visible_in_slice(&tree_slices.up, &target_tree)
        || tree_visible_in_slice(&tree_slices.down, &target_tree);
}

#[derive(Debug)]
struct TreeVisiblitySlices {
    left: Vec<Tree>,
    right: Vec<Tree>,
    up: Vec<Tree>,
    down: Vec<Tree>,
}

impl TreeVisiblitySlices {
    fn create_for_tree(forest: &Forest, x: usize, y: usize) -> TreeVisiblitySlices {
        let forest_width = forest[0].len();
        let forest_height = forest.len();
        let left_of_tree: Vec<Tree> = if y == 0 {
            Vec::new()
        } else {
            forest[x][..=y - 1].to_vec()
        };
        let right_of_tree: Vec<Tree> = if y == forest_width - 1 {
            Vec::new()
        } else {
            forest[x][y + 1..].to_vec()
        };
        let above_tree: Vec<Tree> = if x == 0 {
            Vec::new()
        } else {
            forest[..=x - 1].iter().map(|row| row[y]).collect()
        };
        let below_tree: Vec<Tree> = if x == forest_height - 1 {
            Vec::new()
        } else {
            forest[x + 1..].iter().map(|row| row[y]).collect()
        };
        return TreeVisiblitySlices {
            left: left_of_tree,
            right: right_of_tree,
            up: above_tree,
            down: below_tree,
        };
    }
}

fn tree_visible_in_slice(slice: &Vec<Tree>, target_tree: &Tree) -> bool {
    return slice.iter().all(|tree| tree.height < target_tree.height);
}

fn tree_score(forest: &Forest, x: usize, y: usize) -> usize {
    let target_tree = forest[x][y];

    let mut tree_slices = TreeVisiblitySlices::create_for_tree(forest, x, y);

    tree_slices.left.reverse();
    tree_slices.up.reverse();

    return tree_slice_score(&tree_slices.left, &target_tree)
        * tree_slice_score(&tree_slices.right, &target_tree)
        * tree_slice_score(&tree_slices.up, &target_tree)
        * tree_slice_score(&tree_slices.down, &target_tree);
}

fn tree_slice_score(slice: &Vec<Tree>, target_tree: &Tree) -> usize {
    let tree_visible_count = match slice
        .iter()
        .position(|tree| tree.height >= target_tree.height)
    {
        Some(tree_index) => tree_index + 1,
        None => slice.len(),
    };
    return tree_visible_count;
}

fn part_1(input: &str) -> Result<usize> {
    let forest = parse_forest(input);

    let forest_width = forest[0].len();
    let forest_height = forest.len();
    println!("Tree is {forest_width} wide and {forest_height} tall");
    let trees_visible_count = (0..forest_height)
        .map(|x| {
            (0..forest_width)
                .map(|y| is_tree_visible(&forest, x, y))
                .filter(|&is_visible| is_visible)
                .count()
        })
        .sum();

    return Ok(trees_visible_count);
}

fn part_2(input: &str) -> usize {
    let forest = parse_forest(input);

    let forest_width = forest[0].len();
    let forest_height = forest.len();
    println!("Tree is {forest_width} wide and {forest_height} tall");
    let max_tree_score = (0..forest_height)
        .map(|x| {
            (0..forest_width)
                .map(|y| tree_score(&forest, x, y))
                .max()
                .unwrap()
        })
        .max()
        .unwrap();

    return max_tree_score;
}

fn parse_forest(input: &str) -> Forest {
    let forest: Forest = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| Tree {
                    height: char.to_digit(10).unwrap(),
                })
                .collect()
        })
        .collect();

    return forest;
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn part_1_given() {
        let input = include_str!("./1_given");

        let tree_count = part_1(input).unwrap();

        assert_eq!(tree_count, 21);
    }

    #[test]
    fn parse_forest_simple() {
        let sample_forest_str = "123\n456";
        let expected_forest = vec![
            vec![Tree { height: 1 }, Tree { height: 2 }, Tree { height: 3 }],
            vec![Tree { height: 4 }, Tree { height: 5 }, Tree { height: 6 }],
        ];

        let forest = parse_forest(sample_forest_str);

        println!("{:?}", forest);
        println!("{:?}", expected_forest);

        assert_eq!(two_forests_equal(forest, expected_forest), true);
    }
    fn two_forests_equal(first: Forest, second: Forest) -> bool {
        first.iter().enumerate().all(|(index, first_row)| {
            println!("first_row: {:?}", first_row);
            let second_row = &second[index];
            println!("second_row: {:?}", second_row);
            first_row
                .iter()
                .all(|first_cell| second_row.contains(first_cell))
        })
    }

    #[test]
    fn tree_edge_visible() {
        let forest = vec![
            vec![Tree { height: 1 }, Tree { height: 2 }, Tree { height: 3 }],
            vec![Tree { height: 4 }, Tree { height: 5 }, Tree { height: 6 }],
        ];

        let tree_visibility = is_tree_visible(&forest, 0, 1);

        assert_eq!(tree_visibility, true);
    }
    #[test]
    fn tree_is_visible() {
        let forest = vec![
            vec![Tree { height: 1 }, Tree { height: 2 }, Tree { height: 3 }],
            vec![Tree { height: 4 }, Tree { height: 10 }, Tree { height: 6 }],
            vec![Tree { height: 4 }, Tree { height: 5 }, Tree { height: 1 }],
        ];

        let tree_visibility = is_tree_visible(&forest, 1, 1);

        assert_eq!(tree_visibility, true);
    }
    #[test]
    fn tree_hidden() {
        let forest = vec![
            vec![Tree { height: 1 }, Tree { height: 2 }, Tree { height: 3 }],
            vec![Tree { height: 4 }, Tree { height: 0 }, Tree { height: 6 }],
            vec![Tree { height: 4 }, Tree { height: 5 }, Tree { height: 1 }],
        ];

        let tree_visibility = is_tree_visible(&forest, 1, 1);

        assert_eq!(tree_visibility, false);
    }
    #[test]
    fn tree_score_basic() {
        let forest = vec![
            vec![Tree { height: 1 }, Tree { height: 2 }, Tree { height: 3 }],
            vec![Tree { height: 4 }, Tree { height: 7 }, Tree { height: 6 }],
            vec![Tree { height: 4 }, Tree { height: 5 }, Tree { height: 1 }],
        ];

        let tree_score = tree_score(&forest, 1, 1);

        assert_eq!(tree_score, 1);
    }
    #[test]
    fn tree_score_longer() {
        let forest = vec![
            vec![Tree { height: 1 }, Tree { height: 2 }, Tree { height: 3 }],
            vec![Tree { height: 10 }, Tree { height: 7 }, Tree { height: 6 }],
            vec![Tree { height: 4 }, Tree { height: 5 }, Tree { height: 1 }],
            vec![Tree { height: 4 }, Tree { height: 10 }, Tree { height: 1 }],
        ];

        let tree_score = tree_score(&forest, 1, 1);

        assert_eq!(tree_score, 2);
    }
    #[test]
    fn part_2_given() {
        let input = include_str!("./1_given");

        let max_count = part_2(input);

        assert_eq!(max_count, 8);
    }

    #[test]
    fn part_2_full() {
        let input = include_str!("./1_real");

        let max_count = part_2(input);

        assert_eq!(max_count, 345744);
    }
}
