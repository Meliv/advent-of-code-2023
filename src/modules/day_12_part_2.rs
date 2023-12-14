use std::{collections::HashSet, str::FromStr};

use itertools::Itertools;
use regex::Regex;

const INPUT_FILE_PATH: &str = "src/inputs/day_12.txt";
const GROUP_REGEX: &str = r"[?|#]*[^\.|\r|\n]";

pub fn run() -> usize {

    let mut result = 0;

    let test_data: Vec<(String, Vec<usize>)> = vec![
        (String::from("???.###????.###????.###????.###????.###"), vec![1,1,3,1,1,3,1,1,3,1,1,3,1,1,3]), // Expect 1, Actual 1
        (String::from(".??..??...?##.?.??..??...?##.?.??..??...?##.?.??..??...?##.?.??..??...?##."), vec![1, 1, 3,1, 1, 3,1, 1, 3,1, 1, 3,1, 1, 3]), // Expect 4, Actual 4
        (String::from("?#?#?#?#?#?#?#???#?#?#?#?#?#?#???#?#?#?#?#?#?#???#?#?#?#?#?#?#???#?#?#?#?#?#?#?"), vec![1, 3, 1, 6,1, 3, 1, 6,1, 3, 1, 6,1, 3, 1, 6,1, 3, 1, 6]), // Expect 1, Actual 1
        (String::from("????.#...#...?????.#...#...?????.#...#...?????.#...#...?????.#...#..."), vec![4,1,1,4,1,1,4,1,1,4,1,1,4,1,1]), // Expect 1, Actual 1
        (String::from("????.######..#####.?????.######..#####.?????.######..#####.?????.######..#####.?????.######..#####."), vec![1,6,5,1,6,5,1,6,5,1,6,5,1,6,5]), // Expect 4, Actual 2
        (String::from("?###??????????###??????????###??????????###??????????###????????"), vec![3,2,1,3,2,1,3,2,1,3,2,1,3,2,1]), // Expect 10, Actual 11
        ];
        for td in test_data {
            println!("x");
            result += SpringField::new(td.0, td.1).get_permutations();
        }

    /*
    let test_data: Vec<(String, Vec<usize>)> = get_input();
    
    for td in test_data {
        result += SpringField::new(td.0, td.1).get_permutations();
    }
    */
    println!("Result {}", result);
    result
}

fn get_input() -> Vec<(String, Vec<usize>)> {
    let input = std::fs::read_to_string(INPUT_FILE_PATH).unwrap();

    // Some proper 2am coding here

    let mut result: Vec<(String, Vec<usize>)> = vec![];
    for line in input.lines() {
        let split: Vec<&str> = line.split_whitespace().collect();
        let x: String = split.first().unwrap().to_string();

        let y: Vec<usize> = split.get(1).unwrap()
        .split(',')
        .filter_map(|s| s.parse().ok()) // Parse each substring into usize
        .collect();

        result.push((x,y));
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
