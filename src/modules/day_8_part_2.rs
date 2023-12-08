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

    let mut current_positions: Vec<&Node> = map
        .iter()
        .filter(|(k, _)| k.ends_with('A'))
        .map(|(_, n)| n)
        .collect_vec();

    let mut loop_is_complete: bool = false;

    while !loop_is_complete {
        for i in instructions.iter() {
            let mut next_positions: Vec<&Node> = vec![];
            let mut destination_endings: String = String::new();

            for current_position in &current_positions {
                let next_position = map
                    .get(current_position.destinations.get(*i).unwrap().as_str())
                    .unwrap();

                next_positions.push(next_position);
                destination_endings.push('Z');
            }

            // Are we done?
            result += 1;
            
            if result as f64 % 1000000. == 1. {
                println!("{}", result);
            }

            loop_is_complete = next_positions.iter().all(|n| n.key.ends_with('Z'));

            if loop_is_complete {
                break;
            } else {
                current_positions = next_positions;
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
                key: String::from(key),
                destinations: vec![left, right],
            },
        );
    }

    map
}

#[derive(Debug)]
struct Node {
    key: String,
    destinations: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day8_part2_test() {
        assert_eq!(run(), 16531);
    }
}
