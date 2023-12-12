use std::str::FromStr;

const INPUT_FILE_PATH: &str = "src/inputs/day_12.txt";

pub fn run() -> usize {
    let _ = std::fs::read_to_string(INPUT_FILE_PATH).unwrap();

    
    let input = String::from("???.###");
    let groups: Vec<usize> = vec![1,1,3];
    
    let result = SpringField::new(input, groups).get_permutations();

    println!("Result {}", result);
    result
}

struct SpringField {
    input: String,
    groups: Vec<usize>,
}


impl SpringField {

    fn new(input: String, groups: Vec<usize>) -> SpringField {
        SpringField {input, groups }
    }

    fn get_permutations(&self) -> usize {
        
        0
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
