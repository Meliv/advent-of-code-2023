use regex::Regex;

static INPUT_FILE_PATH: &str = "src/inputs/day_2.txt";
static MAX_RED: u8 = 12;
static MAX_GREEN: u8 = 13;
static MAX_BLUE: u8 = 14;

static REGEX_STRING: &str = r"(\d+)(\w+);*";

pub fn run() {
    let input = std::fs::read_to_string(INPUT_FILE_PATH).unwrap();

    let mut result: i16 = 0;

    for (index, line) in input.lines().enumerate() {
        if is_line_possible(line) {
            result = result + (index as i16) + 1;
        }
    }

    println!("Result is {result}");
}

fn is_line_possible(line: &str) -> bool {
    let value_split: Vec<&str> = line.split(':').collect();
    let formatted_values = value_split.get(1).unwrap().replace(' ', "");
    let value_groups: Vec<&str> = formatted_values.split(';').collect();

    let exp = Regex::new(REGEX_STRING).unwrap();

    for group in value_groups {
        for captures in exp.captures_iter(group) {
            let number: u8 = captures.get(1).unwrap().as_str().parse().unwrap();
            let colour: &str = captures.get(2).unwrap().as_str();

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
    }

    true
}
