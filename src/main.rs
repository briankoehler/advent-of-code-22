use crate::utils::Solution;

mod exercises;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let solution = exercises::day_4::Day4::new(String::from("./inputs/day_4.txt"));
    println!("Part 1 Answer: {}", solution.part_1()?);
    println!("Part 2 Answer: {}", solution.part_2()?);
    Ok(())
}
