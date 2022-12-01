use std::io::{Result, Lines, BufReader, BufRead};
use std::fs::File;
use std::path::Path;

pub trait Solution {
    fn new(input_path: String) -> Self;
    fn part_1(&self) -> u64;
    fn part_2(&self) -> u64;
}

pub fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
