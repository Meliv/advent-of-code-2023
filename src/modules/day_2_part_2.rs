use regex::Regex;

static INPUT_FILE_PATH: &str = "src/inputs/day_2.txt";
static MAX_RED: u8 = 12;
static MAX_GREEN: u8 = 13;
static MAX_BLUE: u8 = 14;

static REGEX_STRING: &str = r"(\d+)(\s)(\w+)*";

pub fn run() -> usize {
    let input = std::fs::read_to_string(INPUT_FILE_PATH).unwrap();

    let result: usize = input.lines().map(get_power).sum();
    println!("Total power is {result}");

    result
}

fn get_power(line: &str) -> usize {
    let exp = Regex::new(REGEX_STRING).unwrap();

    let mut highest_red: usize = 0;
    let mut highest_blue: usize = 0;
    let mut highest_green: usize = 0;

    for captures in exp.captures_iter(line) {
        let number: usize = captures.get(1).unwrap().as_str().parse().unwrap();
        let colour: &str = captures.get(3).unwrap().as_str();

        match colour {
            "red" => {
                if number > highest_red {
                    highest_red = number;
                }
            }
            "blue" => {
                if number > highest_blue {
                    highest_blue = number;
                }
            }
            "green" => {
                if number > highest_green {
                    highest_green = number;
                }
            }
            _ => {}
        }
    }

    highest_blue * highest_red * highest_green
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day2_part2_test() {
        assert_eq!(run(), 70768);
    }
}
