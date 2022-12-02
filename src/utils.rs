use std::io::{Lines, BufReader, BufRead};
use std::fs::File;
use std::path::Path;

pub trait Solution {
    fn new(input_path: String) -> Self;
    fn part_1(&self) -> Result<u64, Box<dyn std::error::Error>>;
    fn part_2(&self) -> Result<u64, Box<dyn std::error::Error>>;
}

pub fn read_lines<P>(filename: P) -> std::io::Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
