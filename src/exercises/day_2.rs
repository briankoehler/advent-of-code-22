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
            let matchup = line?;
            let choices: Vec<&str> = matchup.split(" ").collect();

            let opponent_choice = Choice::from_str(choices[0])?;
            let my_choice = Choice::from_str(choices[1])?;

            let outcome_bonus = match opponent_choice {
                opponent_choice if my_choice == opponent_choice => Outcome::Draw as u64,
                opponent_choice if opponent_choice == my_choice.get_win() => Outcome::Win as u64,
                _ => Outcome::Loss as u64,
            };
            score = score + my_choice as u64 + outcome_bonus;
        }
        Ok(score)
    }

    fn part_2(&self) -> Result<u64, Box<dyn std::error::Error>> {
        let mut score = 0;
        let lines = read_lines(&self.input_path)?;

        for line in lines {
            let matchup = line?;
            let tokens: Vec<&str> = matchup.split(" ").collect();

            let opponent_choice = Choice::from_str(tokens[0])?;
            let outcome = Outcome::from_str(tokens[1])?;

            let my_choice_bonus = match outcome {
                Outcome::Loss => opponent_choice.get_win() as u64,
                Outcome::Draw => opponent_choice as u64,
                Outcome::Win => opponent_choice.get_loss() as u64,
            };
            score = score + outcome as u64 + my_choice_bonus;
        }
        Ok(score)
    }
}
