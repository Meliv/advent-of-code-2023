use regex::Regex;

static INPUT_FILE_PATH: &str = "src/inputs/day_5.txt";

pub fn run() -> usize {
    let lines = std::fs::read_to_string(INPUT_FILE_PATH).unwrap();
    
    lines.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day5_part1_test() {
        assert_eq!(run(), 35);
    }
}
