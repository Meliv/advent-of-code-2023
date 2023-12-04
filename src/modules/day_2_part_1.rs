use regex::Regex;

static INPUT_FILE_PATH: &str = "src/inputs/day_2.txt";
static MAX_RED: usize = 12;
static MAX_GREEN: usize = 13;
static MAX_BLUE: usize = 14;

static REGEX_STRING: &str = r"(\d+)(\s)(\w+)*";

pub fn run() -> usize {
    std::fs::read_to_string(INPUT_FILE_PATH)
        .unwrap()
        .lines()
        .enumerate()
        .map(|(i, f)| if is_line_possible(f) { i + 1 } else { 0 })
        .sum()
}

fn is_line_possible(line: &str) -> bool {
    let exp = Regex::new(REGEX_STRING).unwrap();

    for captures in exp.captures_iter(line) {
        let number: usize = captures.get(1).unwrap().as_str().parse().unwrap();
        let colour: &str = captures.get(3).unwrap().as_str();

        match colour {
            "red" => {
                if number > MAX_RED {
                    return false;
                }
            }
            "blue" => {
                if number > MAX_BLUE {
                    return false;
                }
            }
            "green" => {
                if number > MAX_GREEN {
                    return false;
                }
            }
            _ => {}
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day2_part1_test() {
        assert_eq!(run(), 2563);
    }
}
