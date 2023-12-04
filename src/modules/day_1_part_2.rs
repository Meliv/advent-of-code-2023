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
    let exp = Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine|ten").unwrap();
    let mut start = 0;
    let mut matches: Vec<&str> = vec![];

    while let Some(captures) = exp.captures(&line[start..]) {
        matches.push(captures.get(0).unwrap().as_str());
        start += 1;
    }

    format!("{}{}", match_to_number(matches.first().unwrap()), match_to_number(matches.last().unwrap()))
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
