use regex::Regex;

static INPUT_FILE_PATH: &str = "src/inputs/day_3.txt";

static REGEX_STRING: &str = r"(\d+)(\w+);*";

pub fn run() {
    let input = std::fs::read_to_string(INPUT_FILE_PATH).unwrap();

    let result: u32 = 100;
    
    println!("Total: {result}");
}