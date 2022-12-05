use std::str::FromStr;
use crate::utils::{Solution, read_lines, solution_check_test};

struct Assignment {
    pub lower: u32,
    pub upper: u32,
}

impl FromStr for Assignment {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<_> = s.split("-").collect();
        return Ok(Self {
            lower: tokens[0].parse::<u32>().expect("Invalid assignment"),
            upper: tokens[1].parse::<u32>().expect("Invalid assignment"),
        })
    }
}

pub struct Day4 {
    input_path: String,
}

impl Solution for Day4 {
    fn new(input_path: String) -> Self {
        Self { input_path }
    }

    fn part_1(&self) -> Result<String, Box<dyn std::error::Error>> {
        let lines = read_lines(&self.input_path)?;

        let mut count = 0;
        for line in lines {
            let line = line?;
            let pairs: Vec<_> = line.split(",").collect();
            let assignment_1 = Assignment::from_str(pairs[0])?;
            let assignment_2 = Assignment::from_str(pairs[1])?;

            if (assignment_1.lower <= assignment_2.lower && assignment_1.upper >= assignment_2.upper)
                || (assignment_2.lower <= assignment_1.lower && assignment_2.upper >= assignment_1.upper) {
                count += 1;
            }
        }

        Ok(count.to_string())
    }

    fn part_2(&self) -> Result<String, Box<dyn std::error::Error>> {
        let lines = read_lines(&self.input_path)?;

        let mut count = 0;
        for line in lines {
            let line = line?;
            let pairs: Vec<_> = line.split(",").collect();
            let assignment_1 = Assignment::from_str(pairs[0])?;
            let assignment_2 = Assignment::from_str(pairs[1])?;

            if (assignment_2.lower <= assignment_1.lower && assignment_1.lower <= assignment_2.upper)
                || (assignment_1.lower <= assignment_2.lower && assignment_2.lower <= assignment_1.upper) {
                count += 1;
            }
        }

        Ok(count.to_string())
    }
}

solution_check_test!(Day4, "day_4.txt", "2", "4");
