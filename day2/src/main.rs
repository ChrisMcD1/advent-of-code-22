use std::fs;
fn main() {
    println!("Hello, world!");
    part_1();
    part_2();
}

fn part_1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let games = input
        .split("\n")
        .map(|game_str| Game::from_string(game_str))
        .flatten();
    let final_score: GameScore = games.map(|game| game.get_game_outcome()).sum();
    println!("Final score should be: {:?}", final_score);
}

fn part_2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let games = input
        .split("\n")
        .map(|game_str| Game::from_strategic_string(game_str))
        .flatten();
    let final_score: GameScore = games.map(|game| game.get_game_outcome()).sum();
    println!("Final score should be: {:?}", final_score);
}

struct Game {
    our: Move,
    opponent: Move,
}

impl Game {
    fn from_strategic_string(string: &str) -> Option<Self> {
        let mut moves: Vec<&str> = string.split(" ").collect();
        let our_string = moves.pop()?;
        let opponent_string = moves.pop()?;
        let opponent = match opponent_string {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => panic!("Found an unknown opoponent string"),
        };
        let our = match our_string {
            "X" => match opponent {
                Move::Rock => Move::Scissors,
                Move::Paper => Move::Rock,
                Move::Scissors => Move::Paper,
            },
            "Y" => match opponent {
                Move::Rock => Move::Rock,
                Move::Paper => Move::Paper,
                Move::Scissors => Move::Scissors,
            },
            "Z" => match opponent {
                Move::Rock => Move::Paper,
                Move::Paper => Move::Scissors,
                Move::Scissors => Move::Rock,
            },
            _ => panic!("Found an unknown our string"),
        };
        Some(Game { our, opponent })
    }
    fn from_string(string: &str) -> Option<Self> {
        let mut moves: Vec<&str> = string.split(" ").collect();
        let our_string = moves.pop()?;
        let opponent_string = moves.pop()?;
        let our = match our_string {
            "X" => Move::Rock,
            "Y" => Move::Paper,
            "Z" => Move::Scissors,
            _ => panic!("Found an unknown our string"),
        };
        let opponent = match opponent_string {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => panic!("Found an unknown opoponent string"),
        };
        Some(Game { our, opponent })
    }
    fn get_game_outcome(self) -> GameScore {
        match self.our {
            Move::Rock => {
                1 + match self.opponent {
                    Move::Rock => 3,
                    Move::Paper => 0,
                    Move::Scissors => 6,
                }
            }
            Move::Paper => {
                2 + match self.opponent {
                    Move::Rock => 6,
                    Move::Paper => 3,
                    Move::Scissors => 0,
                }
            }
            Move::Scissors => {
                3 + match self.opponent {
                    Move::Rock => 0,
                    Move::Paper => 6,
                    Move::Scissors => 3,
                }
            }
        }
    }
}

#[derive(Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

type GameScore = i32;
