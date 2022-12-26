use std::io::{Lines, BufReader, BufRead};
use std::fs::File;
use std::path::Path;

pub trait Solution {
    fn new(input_path: String) -> Self where Self: Sized;
    fn part_1(&self) -> Result<String, Box<dyn std::error::Error>>;
    fn part_2(&self) -> Result<String, Box<dyn std::error::Error>>;
}

pub fn read_lines<P>(filename: P) -> std::io::Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

macro_rules! solution_check_test {
    ($day: ident, $file: expr, $p1_answer: expr, $p2_answer: expr) => {
        #[cfg(test)]
        mod tests {
            use super::*;
            
            #[test]
            fn test_solution() -> Result<(), Box<dyn std::error::Error>> {
                let filename = String::from(format!("./inputs/tests/{}", $file));
                if !std::path::Path::new(&filename).exists() {
                    println!("Test file not found.");
                    return Ok(())
                }
                let solution = $day::new(filename);
                assert_eq!(solution.part_1()?, $p1_answer);
                assert_eq!(solution.part_2()?, $p2_answer);
                Ok(())
            }
        }
    };
}

pub(crate) use solution_check_test;
