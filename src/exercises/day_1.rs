use crate::utils::{read_lines, Solution};
use std::cmp;

pub struct Day1 {
    pub input_path: String,
}

impl Solution for Day1 {
    fn new(input_path: String) -> Self {
        Self { input_path }
    }

    fn part_1(&self) -> Result<u64, Box<dyn std::error::Error>> {
        let mut max = 0;
        let mut current_sum = 0;

        if let Ok(lines) = read_lines(&self.input_path) {
            for line in lines {
                if let Ok(weight) = line {
                    if weight.is_empty() {
                        max = cmp::max(max, current_sum);
                        current_sum = 0;
                        continue;
                    }

                    let value: u32 = weight.parse()?;
                    current_sum = current_sum + value;
                }
            }
        }
        Ok(max.into())
    }

    fn part_2(&self) -> Result<u64, Box<dyn std::error::Error>> {
        let mut top_three: Vec<u32> = vec![];
        let mut current_sum = 0;

        if let Ok(lines) = read_lines(&self.input_path) {
            for line in lines {
                if let Ok(weight) = line {
                    if weight.is_empty() {
                        if top_three.len() != 3 {
                            top_three.push(current_sum);
                        }
                        else {
                            let min_pair = top_three
                                .iter()
                                .enumerate()
                                .min_by(|(_, a), (_, b)| a.cmp(b))
                                .unwrap();
                            if &current_sum > min_pair.1 {
                                top_three.remove(min_pair.0);
                                top_three.push(current_sum);
                            }
                        }

                        current_sum = 0;
                        continue;
                    }

                    let value: u32 = weight.parse()?;
                    current_sum = current_sum + value;
                }
            }
        }
        let total: u32 = top_three.iter().sum();
        Ok(total.into())
    }
}
