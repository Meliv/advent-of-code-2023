use itertools::{self, Itertools};
use regex::Regex;
use std::{collections::HashSet, str::FromStr};

const INPUT_FILE_PATH: &str = "src/inputs/day_13.txt";

pub fn run() -> usize {
    let input = std::fs::read_to_string(INPUT_FILE_PATH).unwrap();
    let mut result = 0;

    let fields: Vec<String> = input
        .split_terminator("\n\r\n")
        .map(|s| s.to_string())
        .collect();

    for field in fields {
        let r = horizontal(&field);

        match r {
            Some(r) => result += r * 100,
            None => result += vertical(&field),
        }
    }

    println!("Result {}", result);
    result
}

fn vertical(input: &str) -> usize {
    let mut rotated_input_chars: Vec<Vec<char>> = vec![];

    let mut i = 0;
    while i < input.lines().next().unwrap().len() {
        let mut new_line: Vec<char> = vec![];
        for line in input.lines() {
            let c = line.chars().nth(i).unwrap();
            new_line.push(c);
        }
        rotated_input_chars.push(new_line);
        i += 1;
    }

    let rotated_input: String = rotated_input_chars
        .iter()
        .flat_map(|inner_vec| inner_vec.iter())
        .collect();

    println!("Vertical");
    println!("{}", input);

    horizontal(&rotated_input).unwrap() // Assuming because it wasn't horizontal it must be vertical
}

fn horizontal(input: &str) -> Option<usize> {

    println!("Horizontal");
    println!("{}", input);

    input
        .lines()
        .enumerate()
        .tuple_windows()
        .find_map(|(a, b)| if a.1 == b.1 { Some(b.0) } else { None })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day12_part1_test() {
        assert_eq!(run(), 405);
    }
}
