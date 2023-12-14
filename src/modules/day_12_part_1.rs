use std::{collections::HashSet, str::FromStr};

use regex::Regex;

const INPUT_FILE_PATH: &str = "src/inputs/day_12.txt";
const GROUP_REGEX: &str = r"[?|#]*[^\.|\r|\n]";

pub fn run() -> usize {
    //let _ = std::fs::read_to_string(INPUT_FILE_PATH).unwrap();

    let test_data: Vec<(String, Vec<usize>)> = vec![
        //(String::from("???.###"), vec![1,1,3]), // Expect 1, Actual 1
        (String::from(".??..??...?##."), vec![1, 1, 3]), // Expect 4, Actual 4
                                                         //(String::from("?#?#?#?#?#?#?#?"), vec![1, 3, 1, 6]), // Expect 1, Actual 1
                                                         //(String::from("????.#...#..."), vec![4,1,1]), // Expect 1, Actual 1
                                                         //(String::from("????.######..#####."), vec![1,6,5]), // Expect 4, Actual 2
                                                         //(String::from("?###????????"), vec![3,2,1]), // Expect 10, Actual 11
    ];

    //?#?#?#?#?#?#?#?

    //.#.###.#.######

    let mut result = 0;

    for td in test_data {
        result += SpringField::new(td.0, td.1).get_permutations();
    }

    println!("Result {}", result);
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
        
        if groups.is_empty(){
            return;
        }

        while start < input.len()
        {
            let mut updated_string_vec: Vec<char> = input.chars().collect();
            // Find end of group (you already know start)
            let end = match input
                .chars()
                .enumerate()
                .find(|(i, c)| i > &start && c == &'.')
            {
                Some(n) => n.0,
                None => input.len() - 1,
            };

            let g = *groups.first().unwrap();

            // Do the insert
            if end - start >= g { // Has to be #1 because .###./.???. with 3 resolves to 2, not correct

                // Hack
                if updated_string_vec[start+g] == '.' {
                    return;
                }

                updated_string_vec[start..start + g].fill('#');

                if start + g < input.len() {
                    if updated_string_vec[start + g] == '#' {
                        start += 1;
                        continue;
                    }
                    updated_string_vec[start + g] = '.'; // Concern here
                }
            }
            else {
                return;
            }

            let mut updated_string: String = updated_string_vec.iter().collect();

            let next = updated_string
                .char_indices()
                .find(|x| x.0 > start + g && (x.1 == '?' || x.1 == '#'));

            match next {
                Some(n) => {
                    self.calculate_permutations(updated_string, n.0, groups[1..].to_vec());
                }
                None => {
                    if updated_string.chars().filter(|c| c == &'#').count()
                        == self.groups.iter().sum()
                    {
                        updated_string = updated_string.replace('?', ".");
                        println!("{}", updated_string);
                        self.permutations.insert(updated_string);
                    }
                }
            }
            start += 1;
        }
        /*

        This whole code has gotten ridiculous

        Here's all you need to do
        Given input of //.??.??.### [1,1,3]
        1. Find the first group of chars (.find(?/#) for the start, .find(.) for the end (end of line if not found))
            1b. Insert into that group
            1c. Insert a . after what you just inserted. If that char is a '#', then continue.
            1d. Recursively call the method, passing in the next self.group and the next start (.find(?/#) from end of group you just placed into)

        2. If the group of ?# chars is bigger than the group you're working with, find the next group
        3. If you're ever about to overflow the string, continue. Maybe return. Probably return as incrementing
           count is almost certainly not going to cause it all to fit this time


         */

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day12_part1_test() {
        assert_eq!(run(), 0);
    }
}
