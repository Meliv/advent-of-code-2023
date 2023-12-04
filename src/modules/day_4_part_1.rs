use core::num;
use std::thread::current;

use regex::Regex;

static INPUT_FILE_PATH: &str = "src/inputs/day_4.txt";

static REGEX_GET_NUMBERS: &str = r"\d+";

pub fn run() -> u32 {
    let input = std::fs::read_to_string(INPUT_FILE_PATH).unwrap();
    let exp = Regex::new(REGEX_GET_NUMBERS).unwrap();
    let mut result: u32 = 0;

    for (i, current_line) in input.lines().enumerate() {
        let mut line_points: u32 = 0;
        let line_without_game = current_line.split(':').nth(1).unwrap();
        let splits: Vec<&str> = line_without_game.split("|").collect();
        let win_nos = splits.get(0).unwrap();
        let your_nos: Vec<&str> = splits.get(1).unwrap().split(' ').collect();

        for c in exp.captures_iter(win_nos) {


            if your_nos.contains(&c.get(0).unwrap().as_str()) {

                
                if line_points == 0 {
                    line_points += 1
                } else {
                    line_points *= 2
                }
            }
        }
        result += line_points;
    }

    println!("Total: {}", result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day4_part1_test() {
        assert_eq!(run(), 22193);
    }
}
