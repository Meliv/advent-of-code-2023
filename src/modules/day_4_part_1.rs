use regex::Regex;

static INPUT_FILE_PATH: &str = "src/inputs/day_4.txt";

static REGEX_GET_NUMBERS: &str = r"\b\d+";

pub fn run() -> u32 {
    let input = std::fs::read_to_string(INPUT_FILE_PATH).unwrap();
    let star_exp = Regex::new(REGEX_GET_STARS).unwrap();
    let mut result: u32 = 0;

    for (i, current_line) in input.lines().enumerate() {

        for captures in star_exp.captures_iter(current_line) {
            
        }
    }
    println!("Total: {result}");

    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day4_part1_test() {
        assert_eq!(run(), 13);
    }
}