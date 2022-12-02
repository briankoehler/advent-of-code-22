use crate::utils::{read_lines, Solution};
use std::{collections::HashMap, str::FromStr};

pub struct Day2 {
    pub input_path: String,
    scores: HashMap<&'static str, u64>,
}

impl Solution for Day2 {
    fn new(input_path: String) -> Self {
        Self {
            input_path,
            scores: HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]),
        }
    }
    
    fn part_1(&self) -> u64 {
        let scores = &self.scores;
        let equivalents = HashMap::from([("X", "A"), ("Y", "B"), ("Z", "C")]);
        let wins = HashMap::from([("X", "C"), ("Y", "A"), ("Z", "B")]);

        let mut score = 0;

        if let Ok(lines) = read_lines(&self.input_path) {
            for line in lines {
                if let Ok(matchup) = line {
                    let choices: Vec<&str> = matchup.split(" ").collect();
                    let opponent_choice = choices[0];
                    let my_choice = choices[1];

                    // Draw
                    if equivalents[my_choice] == opponent_choice {
                        score = score + 3;
                    }
                    // Win
                    if wins[my_choice] == opponent_choice {
                        score = score + 6;
                    }
                    score = score + scores[my_choice];
                }
            }
        }
        score
    }

    fn part_2(&self) -> u64 {
        let scores = &self.scores;
        let equivalents = HashMap::from([("A", "X"), ("B", "Y"), ("C", "Z")]);
        let wins = HashMap::from([("C", "X"), ("A", "Y"), ("B", "Z")]);
        let losses = HashMap::from([("C", "Y"), ("A", "Z"), ("B", "X")]);

        let mut score = 0;

        if let Ok(lines) = read_lines(&self.input_path) {
            for line in lines {
                if let Ok(matchup) = line {
                    let tokens: Vec<&str> = matchup.split(" ").collect();
                    let opponent_choice = tokens[0];
                    let outcome = tokens[1];

                    match outcome {
                        "X" => score = score + scores[losses[opponent_choice]],
                        "Y" => {
                            score = score + scores[equivalents[opponent_choice]];
                            score = score + 3;
                        },
                        "Z" => {
                            score = score + scores[wins[opponent_choice]];
                            score = score + 6;
                        }
                        _ => {}
                    }
                }
            }
        }
        score
    }
}
