use regex::Regex;

static INPUT_FILE_PATH: &str = "src/inputs/day_2.txt";
static MAX_RED: u8 = 12;
static MAX_GREEN: u8 = 13;
static MAX_BLUE: u8 = 14;

static REGEX_STRING: &str = r"(\d+)(\w+);*";

pub fn run() {
    let input = std::fs::read_to_string(INPUT_FILE_PATH).unwrap();

    let result: u32 = input.lines().map(get_power).sum();
    println!("Total power is {result}");
}

fn get_power(line: &str) -> u32 {
    let value_split: Vec<&str> = line.split(':').collect();
    let formatted_values = value_split.get(1).unwrap().replace(' ', "");
    let value_groups: Vec<&str> = formatted_values.split(';').collect();

    let exp = Regex::new(REGEX_STRING).unwrap();

    let mut highest_red: u32 = 0;
    let mut highest_blue: u32 = 0;
    let mut highest_green: u32 = 0;

    for group in value_groups {
        for captures in exp.captures_iter(group) {
            let number: u32 = captures.get(1).unwrap().as_str().parse().unwrap();
            let colour: &str = captures.get(2).unwrap().as_str();

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
    }

    highest_blue * highest_red * highest_green
}
