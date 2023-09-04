#![deny(clippy::unwrap_used)]
use std::str::FromStr;
use std::time::Instant;
mod forest;
use forest::*;

fn main() {
    println!("Hello, world!");
    let real_input = include_str!("./1_real");

    if let Ok(tree_count) = part_1(real_input) {
        println!("There are {tree_count} trees");
    } else {
        println!("Failed to parse input 1");
    }

    let start = Instant::now();
    if let Ok(best_score) = part_2(real_input) {
        let end = Instant::now();
        let elapsed = end - start;
        println!("Ran pant 2 in {:?}", elapsed);
        println!("The best tree house has a score of {best_score}");
    } else {
        println!("Failed on input 2");
    }
}

fn is_tree_visible(forest: &Forest, x: usize, y: usize) -> bool {
    let target_tree = forest.trees[x][y];

    let tree_slices = TreeVisiblitySlices::create_for_tree(forest, x, y);

    tree_visible_in_slice(&tree_slices.left, &target_tree)
        || tree_visible_in_slice(&tree_slices.right, &target_tree)
        || tree_visible_in_slice(&tree_slices.up, &target_tree)
        || tree_visible_in_slice(&tree_slices.down, &target_tree)
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
        let left_of_tree: Vec<Tree> = if y == 0 {
            Vec::new()
        } else {
            forest.trees[x][..=y - 1].to_vec()
        };
        let right_of_tree: Vec<Tree> = if y == forest.width() - 1 {
            Vec::new()
        } else {
            forest.trees[x][y + 1..].to_vec()
        };
        let above_tree: Vec<Tree> = if x == 0 {
            Vec::new()
        } else {
            forest.trees[..=x - 1].iter().map(|row| row[y]).collect()
        };
        let below_tree: Vec<Tree> = if x == forest.height() - 1 {
            Vec::new()
        } else {
            forest.trees[x + 1..].iter().map(|row| row[y]).collect()
        };
        TreeVisiblitySlices {
            left: left_of_tree,
            right: right_of_tree,
            up: above_tree,
            down: below_tree,
        }
    }
}

fn tree_visible_in_slice(slice: &[Tree], target_tree: &Tree) -> bool {
    slice.iter().all(|tree| tree.height < target_tree.height)
}

fn tree_score(forest: &Forest, x: usize, y: usize) -> usize {
    let target_tree = forest.trees[x][y];

    let mut tree_slices = TreeVisiblitySlices::create_for_tree(forest, x, y);

    tree_slices.left.reverse();
    tree_slices.up.reverse();

    trees_visible_in_line(&tree_slices.left, &target_tree)
        * trees_visible_in_line(&tree_slices.right, &target_tree)
        * trees_visible_in_line(&tree_slices.up, &target_tree)
        * trees_visible_in_line(&tree_slices.down, &target_tree)
}

fn trees_visible_in_line(line: &[Tree], target_tree: &Tree) -> usize {
    match line
        .iter()
        .position(|tree| tree.height >= target_tree.height)
    {
        Some(tree_index) => tree_index + 1,
        None => line.len(),
    }
}

fn part_1(input: &str) -> Result<usize, ForestParseError> {
    let forest = Forest::from_str(input)?;

    println!(
        "Tree is {} wide and {} tall",
        forest.width(),
        forest.height()
    );
    let trees_visible_count = (0..forest.height())
        .map(|x| {
            (0..forest.width())
                .map(|y| is_tree_visible(&forest, x, y))
                .filter(|&is_visible| is_visible)
                .count()
        })
        .sum();

    Ok(trees_visible_count)
}

fn part_2(input: &str) -> Result<usize, ForestParseError> {
    let forest = Forest::from_str(input)?;

    println!(
        "Tree is {} wide and {} tall",
        forest.width(),
        forest.height()
    );

    Ok((0..forest.height())
        .map(|x| {
            (0..forest.width())
                .map(|y| tree_score(&forest, x, y))
                .max()
                .unwrap_or(0)
        })
        .max()
        .unwrap_or(0))
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
        let expected_forest = Forest::new(vec![
            vec![Tree { height: 1 }, Tree { height: 2 }, Tree { height: 3 }],
            vec![Tree { height: 4 }, Tree { height: 5 }, Tree { height: 6 }],
        ]);

        let forest = Forest::from_str(sample_forest_str).unwrap();

        println!("{:?}", forest);
        println!("{:?}", expected_forest);

        assert_eq!(two_forests_equal(forest, expected_forest), true);
    }
    fn two_forests_equal(first: Forest, second: Forest) -> bool {
        first.trees.iter().enumerate().all(|(index, first_row)| {
            println!("first_row: {:?}", first_row);
            let second_row = &second.trees[index];
            println!("second_row: {:?}", second_row);
            first_row
                .iter()
                .all(|first_cell| second_row.contains(first_cell))
        })
    }

    #[test]
    fn tree_edge_visible() {
        let forest = Forest::new(vec![
            vec![Tree { height: 1 }, Tree { height: 2 }, Tree { height: 3 }],
            vec![Tree { height: 4 }, Tree { height: 5 }, Tree { height: 6 }],
        ]);

        let tree_visibility = is_tree_visible(&forest, 0, 1);

        assert_eq!(tree_visibility, true);
    }
    #[test]
    fn tree_is_visible() {
        let forest = Forest::new(vec![
            vec![Tree { height: 1 }, Tree { height: 2 }, Tree { height: 3 }],
            vec![Tree { height: 4 }, Tree { height: 10 }, Tree { height: 6 }],
            vec![Tree { height: 4 }, Tree { height: 5 }, Tree { height: 1 }],
        ]);

        let tree_visibility = is_tree_visible(&forest, 1, 1);

        assert_eq!(tree_visibility, true);
    }
    #[test]
    fn tree_hidden() {
        let forest = Forest::new(vec![
            vec![Tree { height: 1 }, Tree { height: 2 }, Tree { height: 3 }],
            vec![Tree { height: 4 }, Tree { height: 0 }, Tree { height: 6 }],
            vec![Tree { height: 4 }, Tree { height: 5 }, Tree { height: 1 }],
        ]);

        let tree_visibility = is_tree_visible(&forest, 1, 1);

        assert_eq!(tree_visibility, false);
    }
    #[test]
    fn tree_score_basic() {
        let forest = Forest::new(vec![
            vec![Tree { height: 1 }, Tree { height: 2 }, Tree { height: 3 }],
            vec![Tree { height: 4 }, Tree { height: 7 }, Tree { height: 6 }],
            vec![Tree { height: 4 }, Tree { height: 5 }, Tree { height: 1 }],
        ]);

        let tree_score = tree_score(&forest, 1, 1);

        assert_eq!(tree_score, 1);
    }
    #[test]
    fn tree_score_longer() {
        let forest = Forest::new(vec![
            vec![Tree { height: 1 }, Tree { height: 2 }, Tree { height: 3 }],
            vec![Tree { height: 10 }, Tree { height: 7 }, Tree { height: 6 }],
            vec![Tree { height: 4 }, Tree { height: 5 }, Tree { height: 1 }],
            vec![Tree { height: 4 }, Tree { height: 10 }, Tree { height: 1 }],
        ]);

        let tree_score = tree_score(&forest, 1, 1);

        assert_eq!(tree_score, 2);
    }
    #[test]
    fn part_2_given() {
        let input = include_str!("./1_given");

        let max_count = part_2(input).unwrap();

        assert_eq!(max_count, 8);
    }

    #[test]
    fn part_2_full() {
        let input = include_str!("./1_real");

        let max_count = part_2(input).unwrap();

        assert_eq!(max_count, 345744);
    }
}
