use crate::utils::Solution;

mod exercises;
mod utils;

fn main() {
    let solution = exercises::day_1::Day1 {
        input_path: String::from("./inputs/d1.txt"),
    };
    println!("Part 1 Answer: {}", solution.part_1());
    println!("Part 2 Answer: {}", solution.part_2());
}
