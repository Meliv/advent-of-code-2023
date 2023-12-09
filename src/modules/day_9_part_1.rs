static INPUT_FILE_PATH: &str = "src/inputs/day_9.txt";

pub fn run() -> usize {
    let input = std::fs::read_to_string(INPUT_FILE_PATH).unwrap();

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day9_part1_test() {
        assert_eq!(run(), 114);
    }
}
