use crate::utils::Solution;

mod exercises;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let solution = exercises::day_2::Day2::new(String::from("./inputs/day_2.txt"));
    println!("Part 1 Answer: {}", solution.part_1()?);
    println!("Part 2 Answer: {}", solution.part_2()?);
    Ok(())
}
