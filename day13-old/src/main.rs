use std::str::FromStr;

use anyhow::Result;
fn main() {
    println!("Hello, world!");
}

fn part_1(input: &str) -> Result<usize> {
    unimplemented!();
}

#[derive(Debug, PartialEq)]
enum VecOfVecElem {
    Vec(Vec<VecOfVecElem>),
    Num(u32),
    None,
}

impl FromStr for VecOfVecElem {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // slice off the starting and ending [ ] for the vec
        println!("Considering string {s}");
        let num_parsed = s.parse::<u32>();
        if let Ok(num) = num_parsed {
            return Ok(VecOfVecElem::Num(num));
        }
        let tokens: Vec<&str> = s[1..s.len() - 1].split(",").collect();
        if tokens.len() == 0 {
            return Ok(VecOfVecElem::None);
        }
        let mut vec = vec![];
        for token in tokens {
            vec.push(token.parse().unwrap());
        }
        return Ok(VecOfVecElem::Vec(vec));
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn part_1_given() {
        let input = include_str!("input.given");

        let result = part_1(input).unwrap();

        assert_eq!(result, 13)
    }
    #[test]
    fn simple_array_parse() {
        let input = "[1,2,3]";

        let output: VecOfVecElem = input.parse().unwrap();

        assert_eq!(
            output,
            VecOfVecElem::Vec(vec![
                VecOfVecElem::Num(1),
                VecOfVecElem::Num(2),
                VecOfVecElem::Num(3)
            ])
        );
    }

    #[test]
    fn csv_parse() {
        let input = "[1,2,3]";

        let reader = csv::Reader::from_reader(input.as_bytes());
        let output: Vec<_> = reader
            .records()
            .map(|record| {
                return record.expect("a CSV record");
            })
            .collect();

        assert_eq!(
            output,
            VecOfVecElem::Vec(vec![
                VecOfVecElem::Num(1),
                VecOfVecElem::Num(2),
                VecOfVecElem::Num(3)
            ])
        );
    }

    #[test]
    fn single_nested_parse() {
        let input = "[[1],2,3]";

        let output: VecOfVecElem = input.parse().unwrap();

        assert_eq!(
            output,
            VecOfVecElem::Vec(vec![
                VecOfVecElem::Vec(vec![VecOfVecElem::Num(1)]),
                VecOfVecElem::Num(2),
                VecOfVecElem::Num(3)
            ])
        );
    }

    #[test]
    fn medium_complexity_parse() {
        let input = "[[1],[2,3,4]]";

        let output: VecOfVecElem = input.parse().unwrap();

        assert_eq!(
            output,
            VecOfVecElem::Vec(vec![
                VecOfVecElem::Vec(vec![VecOfVecElem::Num(1)]),
                VecOfVecElem::Vec(vec![
                    VecOfVecElem::Num(2),
                    VecOfVecElem::Num(3),
                    VecOfVecElem::Num(4)
                ])
            ])
        );
    }
}
