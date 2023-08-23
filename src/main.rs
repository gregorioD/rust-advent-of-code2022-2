use std::fmt;
use std::fs::File;
use std::{error::Error, io::Read};

#[derive(Debug, Clone)]
struct ShapeError;

#[derive(PartialEq)]
enum shapes {
    rock,
    paper,
    scissors,
}

impl fmt::Display for ShapeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid play")
    }
}

impl TryFrom<char> for shapes {
    type Error = ShapeError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(shapes::rock),
            'X' => Ok(shapes::rock),
            'B' => Ok(shapes::paper),
            'Y' => Ok(shapes::paper),
            'C' => Ok(shapes::scissors),
            'Z' => Ok(shapes::scissors),
            _ => Err(ShapeError),
        }
    }
}

fn main() {
    let path = String::from(r"strategie.txt");
    let f = File::open(path);
    match (f) {
        Ok(mut file) => {
            let mut file_content = String::new();
            file.read_to_string(&mut file_content).unwrap();
            let rounds = file_content.split("\n");
            let mut total_score = 0;
            for round in rounds {
                let outcome: char = round.chars().nth(2).unwrap_or_else(|| '\0');
                let opponent: char = round.chars().nth(0).unwrap_or_else(|| '\0');
                if outcome != '\0' {
                    total_score +=
                        calculate_predicted_round(outcome, shapes::try_from(opponent).unwrap());
                }
            }
            println!("{}", total_score);
        }
        Err(e) => println!("error: {}", e),
    }
}

fn calculate_round_score(player: shapes, opponent: shapes) -> i32 {
    let mut score = 0;
    match player {
        shapes::rock => score += 1,
        shapes::paper => score += 2,
        shapes::scissors => score += 3,
    }
    calculate_round_result(player, opponent) + score
}

fn calculate_round_result(player: shapes, opponent: shapes) -> i32 {
    match player {
        shapes::rock => {
            if opponent == shapes::scissors {
                return 6;
            } else if opponent == shapes::rock {
                return 3;
            } else {
                return 0;
            }
        }
        shapes::paper => {
            if opponent == shapes::scissors {
                return 0;
            } else if opponent == shapes::rock {
                return 6;
            } else {
                return 3;
            }
        }
        shapes::scissors => {
            if opponent == shapes::scissors {
                return 3;
            } else if opponent == shapes::rock {
                return 0;
            } else {
                return 6;
            }
        }
    }
}

fn calculate_predicted_round(outcome: char, opponent: shapes) -> i32 {
    match opponent {
        // rock 1, paper 2, scissors 3. X lose 6, Y draw 3, Z win 0
        shapes::rock => match outcome {
            'X' => 3,
            'Y' => 4,
            'Z' => 8,
            _ => 0,
        },
        shapes::paper => match outcome {
            'X' => 1,
            'Y' => 5,
            'Z' => 9,
            _ => 0,
        },
        shapes::scissors => match outcome {
            'X' => 2,
            'Y' => 6,
            'Z' => 7,
            _ => 0,
        },
    }
}
