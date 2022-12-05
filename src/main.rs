use std::env;
use utils::Solution;
use exercises::*;

mod exercises;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args().collect();
    let day: usize = args[1].parse().unwrap_or(4);

    let input_path = format!("./inputs/day_{}.txt", day);
    let solution: Box<dyn Solution> = match day {
        1 => Box::new(Day1::new(input_path)),
        2 => Box::new(Day2::new(input_path)),
        3 => Box::new(Day3::new(input_path)),
        4 => Box::new(Day4::new(input_path)),
        _ => panic!("Day not found."),
    };

    run_solution(solution)
}

fn run_solution(solution: Box<dyn Solution>) -> Result<(), Box<dyn std::error::Error>> {
    println!("Part 1 Answer: {}", solution.part_1()?);
    println!("Part 2 Answer: {}", solution.part_2()?);
    Ok(())
}
