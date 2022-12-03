use std::{collections::HashSet};
use crate::utils::{Solution, read_lines};

const LOWERCASE_OFFSET: u32 = 96;
const UPPERCASE_OFFSET: u32 = 65 - 27;

pub struct Day3 {
    input_path: String
}

impl Solution for Day3 {
    fn new(input_path: String) -> Self {
        Self { input_path }
    }

    fn part_1(&self) -> Result<u64, Box<dyn std::error::Error>> {
        let lines = read_lines(&self.input_path)?;

        let mut priority_sum = 0;
        for line in lines {
            let rucksack = line?;

            let compartment_1 = &rucksack[..rucksack.len() / 2];
            let compartment_2 = &rucksack[rucksack.len() / 2..];

            let mut items: Vec<_> = compartment_1.chars().collect();
            let uniques_1: HashSet<_> = HashSet::from_iter(items);
            items = compartment_2.chars().collect();
            let uniques_2: HashSet<_> = HashSet::from_iter(items);
            
            let common: Vec<_> = uniques_1.intersection(&uniques_2).collect();
            let item = common[0];
            match item.is_ascii_uppercase() {
                true => priority_sum = priority_sum + (*item as u32 - UPPERCASE_OFFSET),
                false => priority_sum = priority_sum + (*item as u32 - LOWERCASE_OFFSET),
            }
        }
        Ok(priority_sum.into())
    }

    fn part_2(&self) -> Result<u64, Box<dyn std::error::Error>> {
        let lines = read_lines(&self.input_path)?.into_iter();
        let lines: Vec<_> = lines.collect::<Result<_, _>>()?;

        let mut priority_sum = 0;
        for group in lines.windows(3).step_by(3) {
            let mut items: Vec<_> = group[0].chars().collect();
            let uniques_1: HashSet<_> = HashSet::from_iter(items);
            items = group[1].chars().collect();
            let uniques_2: HashSet<_> = HashSet::from_iter(items);
            items = group[2].chars().collect();
            let uniques_3: HashSet<_> = HashSet::from_iter(items);

            let commons: Vec<_> = uniques_1.iter()
                .filter(|item| uniques_2.contains(item))
                .filter(|item| uniques_3.contains(item))
                .collect();

            let badge = commons[0];
            match badge.is_ascii_uppercase() {
                true => priority_sum = priority_sum + (*badge as u32 - UPPERCASE_OFFSET),
                false => priority_sum = priority_sum + (*badge as u32 - LOWERCASE_OFFSET),
            }
        }

        Ok(priority_sum.into())
    }
}
