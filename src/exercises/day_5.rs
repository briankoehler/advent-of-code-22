use std::collections::{HashMap, hash_map::Entry};
use crate::utils::{Solution, solution_check_test, read_lines};

pub struct Day5 {
    input_path: String,
}

fn get_second_to_last_token(input: impl Into<String>) -> String {
    let s: String = input.into();
    let tokens: Vec<_> = s.split(' ').collect();
    tokens.get(tokens.len() - 2).unwrap().to_string()
}

fn form_str_from_stacks(stacks: HashMap<usize, Vec<char>>, max_value: usize) -> String {
    let mut answer = "".to_string();
    for i in 1..max_value + 1 {
        answer.push(*stacks[&i].last().unwrap());
    }
    answer
}

fn add_chars_to_stack(stacks: &mut HashMap<usize, Vec<char>>, input: String) {
    for (index, character) in input.char_indices() {
        if !character.is_alphabetic() { continue }
        match stacks.entry((index + 3) / 4) {
            Entry::Occupied(mut e) => { e.get_mut().insert(0, character); },
            Entry::Vacant(e) => { e.insert(vec![character]); },
        };
    }
}

fn process_move<F>(input: String, mut f: F) -> Result<(), Box<dyn std::error::Error>>
where
    F: FnMut(usize, usize, usize) -> ()
{
    let tokens: Vec<_> = input.split(' ').collect();
    let count = tokens[1].parse()?;
    let source = tokens[3].parse()?;
    let destination = tokens[5].parse()?;
    Ok(f(count, source, destination))
}

impl Solution for Day5 {
    fn new(input_path: String) -> Self where Self: Sized {
        Self { input_path }
    }

    fn part_1(&self) -> Result<String, Box<dyn std::error::Error>> {
        let lines = read_lines(&self.input_path)?;
        
        let mut stacks: HashMap<usize, Vec<char>> = HashMap::new();
        let mut max_key_value: usize = 0;
        for line in lines {
            let line = line?;
            if line.is_empty() { continue }

            if line.starts_with(" 1") {
                max_key_value = get_second_to_last_token(line).parse()?;
            }
            else if line.starts_with("move") {
                let callback = |count: usize, source: usize, destination: usize| {
                    for _ in 0..count {
                        let mover = stacks.get_mut(&source).unwrap().pop().unwrap();
                        stacks.get_mut(&destination).unwrap().push(mover);
                    }
                };
                process_move(line, callback)?;
            }
            else {
                add_chars_to_stack(&mut stacks, line);
            }
        }

        let answer = form_str_from_stacks(stacks, max_key_value);
        Ok(answer.into())
    }

    fn part_2(&self) -> Result<String, Box<dyn std::error::Error>> {
        let lines = read_lines(&self.input_path)?;
        
        let mut stacks: HashMap<usize, Vec<char>> = HashMap::new();
        let mut max_key_value: usize = 0;
        for line in lines {
            let line = line?;
            if line.is_empty() { continue }

            if line.starts_with(" 1") {
                max_key_value = get_second_to_last_token(line).parse()?;
            }
            else if line.starts_with("move") {
                let callback = |count: usize, source: usize, destination: usize| {
                    let source_stack = stacks.get_mut(&source).unwrap();
                    let mut movers = source_stack.as_slice()[source_stack.len() - count..].to_vec();
                    source_stack.truncate(source_stack.len() - count);
                    let destination_stack = stacks.get_mut(&destination).unwrap();
                    destination_stack.append(&mut movers);
                };
                process_move(line, callback)?;
            }
            else {
                add_chars_to_stack(&mut stacks, line);
            }
        }

        let answer = form_str_from_stacks(stacks, max_key_value);
        Ok(answer.into())
    }
}

solution_check_test!(Day5, "day_5.txt", "CMZ".to_string(), "MCD".to_string());
