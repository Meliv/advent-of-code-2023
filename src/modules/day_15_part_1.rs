use std::iter;

use itertools::Itertools;

const INPUT_FILE_PATH: &str = "src/inputs/day_15.txt";

pub fn run() -> usize {
    //let input = std::fs::read_to_string(INPUT_FILE_PATH).unwrap();
    let mut result = 0;


    let test = "rn=1";

    for c in test.chars() {

        println!("{}", c);

    }


    println!("Result: {}", result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day15_part1_test() {
        assert_eq!(run(), 136);
    }
}
