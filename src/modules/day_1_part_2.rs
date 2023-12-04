use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

static INPUT_FILE_PATH: &str = "src/inputs/day_1.txt";

pub fn run() -> u32 {
    std::fs::read_to_string(INPUT_FILE_PATH)
        .unwrap()
        .lines()
        .map(process_line)
        .sum()
}

fn process_line(line: &str) -> u32 {
    let exp = Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine").unwrap();

    let first = exp.find(line).unwrap().as_str();
    let reversed_line: String = line.chars().rev().collect();
    let last = exp.find(reversed_line.as_str()).unwrap().as_str();

    format!("{}{}", match_to_number(first), match_to_number(last))
        .parse()
        .unwrap()
}

fn match_to_number(m: &str) -> &str {
    match m {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => m,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1_part2_test() {
        assert_eq!(run(), 54980);
    }
}
