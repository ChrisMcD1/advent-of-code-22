use std::time::Instant;

fn main() {
    println!("Hello, world!");
    let start = Instant::now();
    let input = include_str!("../input.prod");

    let output = part_1(input);

    println!("Got {output} in {:?}", start.elapsed());
}

fn part_1(input: &str) -> String {
    let digit: i64 = input.lines().map(|line| snafu_to_digit(line)).sum();
    digit_to_snafu(digit)
}

fn digit_to_snafu(mut val: i64) -> String {
    let mut snafu: Vec<char> = vec![];
    let mut i = 1;
    while val > 0 {
        let power = 5i64;
        let remainder = val % power;
        val = val - remainder;
        let char = match remainder {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => {
                val = val + 6;
                '='
            }
            4 => {
                val = val + 5;
                '-'
            }
            _ => unreachable!(),
        };
        val = val / 5;
        snafu.push(char);
        i = i + 1;
    }
    snafu.into_iter().rev().collect()
}

fn snafu_to_digit(snafu: &str) -> i64 {
    snafu.chars().rev().enumerate().fold(0, |acc, (i, c)| {
        acc + (5i64.pow(i as u32))
            * match c {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '-' => -1,
                '=' => -2,
                _ => unreachable!(),
            }
    })
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn digit_10_to_snafu() {
        let input = 10;

        let snafu = digit_to_snafu(input);

        assert_eq!(snafu, String::from("20"))
    }

    #[test]
    fn snafu_2minus_to_digit() {
        let input = "2-";

        let digit = snafu_to_digit(input);

        assert_eq!(digit, 9);
    }

    #[test]
    fn snafu_20_to_digit() {
        let input = "20";

        let digit = snafu_to_digit(input);

        assert_eq!(digit, 10);
    }

    #[test]
    fn long_snafu_to_digit() {
        let input = "1121-1110-1=0";

        let digit = snafu_to_digit(input);

        assert_eq!(digit, 314159265);
    }

    #[test]
    fn long_digit_to_snafu() {
        let input = 314159265;

        let snafu = digit_to_snafu(input);

        assert_eq!(snafu, "1121-1110-1=0");
    }

    #[test]
    fn part_1_given() {
        let input = include_str!("../input.dev");

        let output = part_1(input);

        assert_eq!(output, String::from("2=-1=0"));
    }
}
