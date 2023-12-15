use std::iter;

use itertools::Itertools;

const INPUT_FILE_PATH: &str = "src/inputs/day_14.txt";

pub fn run() -> usize {
    let input = std::fs::read_to_string(INPUT_FILE_PATH).unwrap();
    let mut result = 0;

    let mut x: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    /*
    let mut has_moved = true;
    while has_moved {
        has_moved = false;
        
        if let (Some(first), Some(second)) = (x.get_mut(0), x.get_mut(1)) {
            if (a, b) = first.iter_mut().zip(second.iter_mut()).next().unwrap() {
                
                *zipped.0 = 'x';
                *zipped.1 = 'u';
                
            }
        }
        
        println!("{:?}", x);
        break;
    }
    // Work something out here
    */
    
    println!("Result: {}", result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day14_part1_test() {
        assert_eq!(run(), 136);
    }
}
