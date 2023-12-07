use core::num;

use regex::Regex;

static INPUT_FILE_PATH: &str = "src/inputs/day_6.txt";

pub fn run() -> usize {
    let _ = std::fs::read_to_string(INPUT_FILE_PATH).unwrap();

    println!("Day 6 Part 1 Result");

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day6_part1_test() {
        assert_eq!(run(), 0);
    }
}
