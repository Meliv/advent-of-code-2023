use std::{collections::HashSet, str::FromStr};

use regex::Regex;

const INPUT_FILE_PATH: &str = "src/inputs/day_12.txt";
const GROUP_REGEX: &str = r"[?|#]*[^\.|\r|\n]";

pub fn run() -> usize {
    //let _ = std::fs::read_to_string(INPUT_FILE_PATH).unwrap();

    //.??..??...?##. 1,1,3
    let input = String::from(".??..??...?##.");
    let groups: Vec<usize> = vec![1,1,3];

    let result = SpringField::new(input, groups).get_permutations();

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

    fn calculate_permutations(
        &mut self,
        input: String,
        start: usize,
        groups: Vec<usize>,
    ) {
        //????.??? 1,1,3
        //.???.### 1,1,3


        for g in &groups {
            
            let mut i: usize = 0;
            let mut updated_string: String = input.clone();
            while i < *g {
                updated_string.replace_range(start+i..start+i+1, "#");
                i += 1;
            }
            updated_string.replace_range(start+i..start+i+1, "."); // Todo, make sure this doesn't overflow when we just replaced the last char in string


            let next = updated_string.char_indices().find(|x| x.0 > start + g && x.1 == '?'); // Todo or #

            match next {
                Some(n) => {
                    self.calculate_permutations(updated_string, n.0, groups[1..].to_vec()); // Loop here?
                }
                None => { // We've reached the end
                    // Valid permutation
                    if updated_string.chars().filter(|c| c == &'#').count() == self.groups.iter().sum() {
                        self.permutations.insert(updated_string);
                    }
                }
            }
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
