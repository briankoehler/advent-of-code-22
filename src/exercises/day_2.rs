use crate::utils::{read_lines, Solution};
use std::{str::FromStr};

#[derive(Debug, PartialEq)]
enum Choice {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for Choice {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "A" | "X" => Ok(Choice::Rock),
            "B" | "Y" => Ok(Choice::Paper),
            "C" | "Z" => Ok(Choice::Scissors),
            _ => Err(String::from("Invalid")),
        }
    }
}

impl Choice {
    fn get_win(&self) -> Choice {
        match *self {
            Choice::Rock => Choice::Scissors,
            Choice::Paper => Choice::Rock,
            Choice::Scissors => Choice::Paper,
        }
    }

    fn get_loss(&self) -> Choice {
        match *self {
            Choice::Rock => Choice::Paper,
            Choice::Paper => Choice::Scissors,
            Choice::Scissors => Choice::Rock,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Outcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

impl FromStr for Outcome {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "X" => Ok(Outcome::Loss),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(String::from("Invalid")),
        }
    }
}

pub struct Day2 {
    pub input_path: String,
}

impl Solution for Day2 {
    fn new(input_path: String) -> Self {
        Self { input_path }
    }
    
    fn part_1(&self) -> Result<u64, Box<dyn std::error::Error>> {
        let mut score = 0;
        let lines = read_lines(&self.input_path)?;

        for line in lines {
            if let Ok(matchup) = line {
                let choices: Vec<&str> = matchup.split(" ").collect();
                let opponent_choice = Choice::from_str(choices[0])?;
                let my_choice = Choice::from_str(choices[1])?;

                if my_choice == opponent_choice {
                    score = score + Outcome::Draw as u64;
                }
                else if my_choice.get_win() == opponent_choice {
                    score = score + Outcome::Win as u64;
                }
                else {
                    score = score + Outcome::Loss as u64;
                }
                score = score + my_choice as u64;
            }
        }
        Ok(score)
    }

    fn part_2(&self) -> Result<u64, Box<dyn std::error::Error>> {
        let mut score = 0;
        let lines = read_lines(&self.input_path)?;

        for line in lines {
            if let Ok(matchup) = line {
                let tokens: Vec<&str> = matchup.split(" ").collect();
                let opponent_choice = Choice::from_str(tokens[0])?;
                let outcome = Outcome::from_str(tokens[1])?;

                match outcome {
                    Outcome::Loss => score = score + opponent_choice.get_win() as u64,
                    Outcome::Draw => {
                        score = score + opponent_choice as u64;
                        score = score + 3;
                    },
                    Outcome::Win => {
                        score = score + opponent_choice.get_loss() as u64;
                        score = score + 6;
                    }
                }
            }
        }
        Ok(score)
    }
}
