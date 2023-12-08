use itertools::Itertools;
use std::collections::HashMap;

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

    let start_positions: Vec<&Node> = map
        .iter()
        .filter(|(k, _)| k.ends_with('A'))
        .map(|(_, n)| n)
        .collect_vec();

    let mut iteration_results: Vec<usize> = vec![];

    for start_position in start_positions {
        let mut iteration_found: bool = false;
        let mut current_position = start_position;
        let mut iteration: usize = 0;

        while !iteration_found {
            for i in instructions.iter() {
                let next_position = map
                    .get(current_position.destinations.get(*i).unwrap().as_str())
                    .unwrap();

                iteration += 1;

                iteration_found = next_position.key.ends_with('Z');

                if iteration_found {
                    break;
                } else {
                    current_position = next_position;
                }
            }
        }

        iteration_results.push(iteration);
    }

    let result: usize = iteration_results.iter().fold(1, |acc, &x| num::integer::lcm(acc, x));

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
        assert_eq!(run(), 24035773251517);
    }
}
