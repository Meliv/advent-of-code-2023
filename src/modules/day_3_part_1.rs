use regex::Regex;

static INPUT_FILE_PATH: &str = "src/inputs/day_3.txt";

static REGEX_STRING: &str = r"(\d+)(\w+);*";

pub fn run() {
    let input = std::fs::read_to_string(INPUT_FILE_PATH).unwrap();

    for (i, current_line) in input.lines().peekable().enumerate() {

        let prev_line = if i != 0 {
            input.lines().nth(i-1).unwrap()
        } else {
            ""
        };
        let next_line = if i != input.lines().count() - 1 {
            input.lines().nth(i + 1).unwrap()
        } else {
            ""
        };
    }

    let result: u32 = 100;
    println!("Total: {result}");
}
