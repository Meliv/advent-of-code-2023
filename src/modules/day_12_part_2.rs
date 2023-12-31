use std::{collections::HashSet, str::FromStr};

use itertools::Itertools;
use regex::Regex;

const INPUT_FILE_PATH: &str = "src/inputs/day_12.txt";
const GROUP_REGEX: &str = r"[?|#]*[^\.|\r|\n]";

pub fn run() -> usize {
    let mut result = 0;

    let test_data: Vec<(String, Vec<usize>)> = get_input();

    let mut counter = 1;
    for td in test_data {
        println!("Count: {}", counter);
        result += SpringField::new(td.0, td.1).get_permutations();
    
        counter += 1;
    }

    println!("Result {}", result);
    result
}

fn get_input() -> Vec<(String, Vec<usize>)> {
    let input = std::fs::read_to_string(INPUT_FILE_PATH).unwrap();
    let mut result: Vec<(String, Vec<usize>)> = vec![];
    for line in input.lines() {
        let split = line.split_once(' ').unwrap();
        let string: String = std::iter::repeat(split.0).take(5).collect_vec().join("?");
        let groups: Vec<usize> = split
            .1
            .split(',')
            .map(str::parse)
            .collect::<Result<Vec<usize>, _>>()
            .unwrap()
            .repeat(5);

        result.push((string, groups));
    }
    result
}

struct SpringField {
    input: String,
    groups: Vec<usize>,
    permutations: HashSet<String>,
}

impl SpringField {
    fn new(input: String, groups: Vec<usize>) -> SpringField {
        SpringField {
            input,
            groups,
            permutations: HashSet::new(),
        }
    }

    fn get_permutations(&mut self) -> usize {
        let start = self.input.find(|c| c == '#' || c == '?').unwrap();
        self.calculate_permutations(self.input.clone(), start, self.groups.clone());
        self.permutations.len()
    }

    fn calculate_permutations(&mut self, input: String, mut start: usize, groups: Vec<usize>) {
        // Edge case city
        if groups.is_empty() {
            if input.chars().filter(|c| c == &'#').count() == self.groups.iter().sum() {
                let updated_string = input.replace('?', ".");
                //println!("{}", updated_string);
                self.permutations.insert(updated_string);
            }
            return;
        }

        while start < input.len() {
            let mut updated_string_vec: Vec<char> = input.chars().collect();

            if updated_string_vec[start] == '.' {
                let next_group: Option<(usize, char)> = find_next_group(updated_string_vec, start);

                match next_group {
                    Some(n) => {
                        start = n.0;
                        continue;
                    }
                    None => {
                        return;
                    }
                }
            }

            // Find end of group (you already know start)
            let end = match input
                .chars()
                .enumerate()
                .find(|(i, c)| i > &start && c == &'.')
            {
                Some(n) => n.0 - 1,
                None => input.len() - 1,
            };

            let g = *groups.first().unwrap();

            // Do the insert
            if (end - start + 1) >= g {
                // Has to be #1 because .###./.???. with 3 resolves to 2, not correct

                updated_string_vec[start..start + g].fill('#');

                if start + g < input.len() {
                    if updated_string_vec[start + g] == '#' {
                        start += 1;
                        continue;
                    }
                    updated_string_vec[start + g] = '.'; // Concern here
                }
            } else {
                start += 1;
                continue;
            }

            let mut updated_string: String = updated_string_vec.iter().collect();

            let next: Option<(usize, char)> = find_next_group(updated_string_vec, start + g);

            match next {
                Some(n) => {
                    self.calculate_permutations(updated_string, n.0, groups[1..].to_vec());
                }
                None => {
                    if updated_string.chars().filter(|c| c == &'#').count()
                        == self.groups.iter().sum()
                    {
                        // Hack
                        if groups.len() == 1 {
                            updated_string = updated_string.replace('?', ".");
                            //println!("{}", updated_string);
                            self.permutations.insert(updated_string);
                        }
                    }
                }
            }
            start += 1;
        }
    }
}

fn find_next_group(chars: Vec<char>, start: usize) -> Option<(usize, char)> {
    let string: String = chars.iter().collect();
    string
        .char_indices()
        .find(|x| x.0 > start && (x.1 == '?' || x.1 == '#'))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day12_part1_test() {
        assert_eq!(run(), 6935);
    }
}
