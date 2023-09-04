use std::str::FromStr;
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Tree {
    pub height: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Forest {
    pub trees: Vec<Vec<Tree>>,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ForestParseError {
    NonDigitCharacter,
}

impl FromStr for Forest {
    type Err = ForestParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trees = s.lines().map(parse_forest_line).collect::<Result<_, _>>()?;
        Ok(Forest::new(trees))
    }
}

fn parse_forest_line(line: &str) -> Result<Vec<Tree>, ForestParseError> {
    line.chars()
        .map(|char| {
            let height = char
                .to_digit(10)
                .ok_or(ForestParseError::NonDigitCharacter)?;
            Ok(Tree { height })
        })
        .collect()
}

impl Forest {
    pub fn width(&self) -> usize {
        self.trees[0].len()
    }
    pub fn height(&self) -> usize {
        self.trees.len()
    }
    pub fn new(trees: Vec<Vec<Tree>>) -> Self {
        Forest { trees }
    }
}
