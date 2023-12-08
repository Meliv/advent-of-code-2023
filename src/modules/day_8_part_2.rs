use core::num;
use itertools::Itertools;
use std::{collections::HashMap, thread::current};

use regex::Regex;

static INPUT_FILE_PATH: &str = "src/inputs/day_8.txt";

pub fn run() -> usize {
    let input = std::fs::read_to_string(INPUT_FILE_PATH).unwrap();

    let instructions: Vec<usize> = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => 0,
            _ => 1,
        })
        .collect_vec();

    let map = get_map(&input);

    let mut result: usize = 0;
    let mut current_position: &str = "AAA";

    while current_position != "ZZZ" {

        for i in instructions.iter() {
            let m = map.get(current_position).unwrap();
            current_position = m.destinations.get(*i).unwrap();
            
            result += 1;

            if current_position == "ZZZ" {
                break;
            }
        }
    }

    println!("Result {}", result);

    result
}

fn get_map(input: &str) -> HashMap<&str, Node> {
    let mut map: HashMap<&str, Node> = HashMap::new();

    for line in input.lines().enumerate().filter(|(i, _)| i > &1) {
        let key = &line.1[0..=2];
        let left = String::from(&line.1[7..=9]);
        let right = String::from(&line.1[12..=14]);
        map.insert(
            key,
            Node {
                destinations: vec![left, right],
            },
        );
    }

    map
}

#[derive(Debug)]
struct Node {
    destinations: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day7_part2_test() {
        assert_eq!(run(), 16531);
    }
}
