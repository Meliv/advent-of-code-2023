use std::fs::File;
use std::io::{self, BufRead, BufReader};

static INPUT_FILE_PATH: &str = "src/inputs/day_1.txt";

pub fn run() -> u32 {
    read_input()
    .unwrap()
    .iter()
    .map(|f| process_line(f.as_str()))
    .sum()
}

fn process_line(line: &str) -> u32 {
    let digits: Vec<char> = line.chars().filter(|&c| c.is_ascii_digit()).collect();

    let first = digits.first().unwrap_or(&'0');
    let last = digits.last().unwrap_or(&'0');

    match format!("{}{}", first, last).parse() {
        Ok(parsed) => parsed,
        _ => 0
    }
}

fn read_input() -> Result<Vec<String>, io::Error> {
    let file = File::open(INPUT_FILE_PATH)?;
    let reader = BufReader::new(file);
    let mut lines = Vec::new();

    for line in reader.lines() {
        lines.push(line?);
    }

    Ok(lines)
}