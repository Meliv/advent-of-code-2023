const INPUT_FILE_PATH: &str = "src/inputs/day_12.txt";

pub fn run() -> usize {
    let input = std::fs::read_to_string(INPUT_FILE_PATH).unwrap();

    let result = 0;

    println!("Result {}", result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day12_part1_test() {
        assert_eq!(run(), 0);
    }
}