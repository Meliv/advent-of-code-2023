use std::{iter, thread::current};

use itertools::Itertools;

const INPUT_FILE_PATH: &str = "src/inputs/day_15.txt";

pub fn run() -> usize {
    let input = std::fs::read_to_string(INPUT_FILE_PATH).unwrap();
    let x: String = input.replace(['\r','\n'], "");
    let replaced_input: Vec<&str> = x.split(',').collect();

    let result_vec: Vec<usize> = replaced_input.iter().map(|s| calculate_string(s)).collect();
    let result = result_vec.iter().sum();

    println!("Result: {}", result);
    result
}

fn calculate_string(input: &str) -> usize {
    let mut result = 0;
    for c in input.chars() {
        result += c as usize;
        result *= 17;
        result %= 256;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day15_part1_test() {
        assert_eq!(run(), 136);
    }
}
