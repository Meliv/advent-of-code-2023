use std::{collections::HashSet, str::FromStr};

use regex::Regex;

const INPUT_FILE_PATH: &str = "src/inputs/day_12.txt";
const GROUP_REGEX: &str = r"[?|#]*[^\.|\r|\n]";

pub fn run() -> usize {
    //let _ = std::fs::read_to_string(INPUT_FILE_PATH).unwrap();

    let test_data: Vec<(String, Vec<usize>)> = vec![
        //(String::from("???.###"), vec![1,1,3]), // Expect 1, Actual 1
        //(String::from(".??..??...?##."), vec![1,1,3]), // Expect 4, Actual 4
        //(String::from("?#?#?#?#?#?#?#?"), vec![1, 3, 1, 6]), // Expect 1, Actual 1
        //(String::from("????.#...#..."), vec![4,1,1]), // Expect 1, Actual 1
        (String::from("????.######..#####."), vec![1,6,5]), // Expect 4, Actual 2
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
        //let input = String::from(".??..??...");
        //let groups: Vec<usize> = vec![1, 2];

        // Start needs to increase here to allow the loop to move over the first group

        while start < input.len() {
            // Almost certainly not right
            let g_opt = groups.first();
            let g = match g_opt {
                Some(g) => g,
                None => return,
            };

            let mut updated_string_vec: Vec<char> = input.chars().collect();

            // Bit of a hack but I got bored of handling off by 1 errors
            if updated_string_vec[start] == '.' {
                return;
            }

            let end = match input
                .chars()
                .enumerate()
                .find(|(i, x)| i > &start && x == &'.')
            {
                Some(n) => n.0 - 1,
                None => input.len() - 1,
            };

            // Something here specifically about when strings end in . and when they don't

            // ??..... // start 0 end 2 group 2
            // ##.....
            // ??..... // start 1 end 2 group 2

            //if start + g <= input.len() && end - start <= *g {
            if end - start + 1 >= *g {
                updated_string_vec[start..start + g].fill('#');

                if start + g < input.len() {
                    if updated_string_vec[start + g] == '#' {
                        start += 1;
                        continue;
                    } else {
                        updated_string_vec[start + g] = '.';
                    }
                }
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
                    // We've reached the end
                    // Valid permutation
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
