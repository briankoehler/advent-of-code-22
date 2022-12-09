use std::collections::HashSet;
use crate::utils::{Solution, solution_check_test, read_lines};

const MARKER_LENGTH: usize = 4;
const MESSAGE_LENGTH: usize = 14;

pub struct Day6 {
    input_path: String,
}

impl Solution for Day6 {
    fn new(input_path: String) -> Self where Self: Sized {
        Self { input_path }
    }

    fn part_1(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut lines = read_lines(&self.input_path)?;
        let datastream = lines.next().unwrap()?;

        let characters: Vec<_> = datastream.chars().collect();
        let mut processed = MARKER_LENGTH;
        for window in characters.windows(MARKER_LENGTH) {
            let mut set: HashSet<char> = HashSet::new();

            for character in window {
                if set.contains(character) {
                    break;
                }
                set.insert(*character);
            }

            if set.len() == MARKER_LENGTH {
                return Ok(processed.to_string());
            }
            
            processed += 1;
        }

        Err("".into())
    }

    fn part_2(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut lines = read_lines(&self.input_path)?;
        let datastream = lines.next().unwrap()?;

        let characters: Vec<_> = datastream.chars().collect();
        let mut processed = MESSAGE_LENGTH;
        for window in characters.windows(MESSAGE_LENGTH) {
            let mut set: HashSet<char> = HashSet::new();

            for character in window {
                if set.contains(character) {
                    break;
                }
                set.insert(*character);
            }

            if set.len() == MESSAGE_LENGTH {
                return Ok(processed.to_string());
            }

            processed += 1;
        }

        Err("".into())
    }
}

solution_check_test!(Day6, "day_6.txt", "7".to_string(), "19".to_string());
