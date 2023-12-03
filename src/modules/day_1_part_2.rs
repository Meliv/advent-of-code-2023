use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

static INPUT_FILE_PATH: &str = "src/inputs/day_1.txt";

pub fn run() -> u32 {
    read_input().unwrap().iter().map(|f| process_line(f.as_str())).sum()
}

fn process_line(line: &str) -> u32 {
    let exp = Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine|ten").unwrap();
    let mut start = 0;
    let mut matches: Vec<&str> = vec![];

    while let Some(captures) = exp.captures(&line[start..]) {
        matches.push(captures.get(0).unwrap().as_str());
        start += 1;
    }

    match format!(
        "{}{}",
        match_to_number(matches.first().copied()),
        match_to_number(matches.last().copied())
    )
    .parse()
    {
        Ok(parsed) => parsed,
        _ => 0,
    }
}

fn match_to_number(m: Option<&str>) -> &str {
    let mut result = "";

    if let Some(val) = m {
        if val.parse::<f64>().is_ok() {
            result = val;
        } else {
            result = match val {
                "one" => "1",
                "two" => "2",
                "three" => "3",
                "four" => "4",
                "five" => "5",
                "six" => "6",
                "seven" => "7",
                "eight" => "8",
                "nine" => "9",
                _ => ""
            };
        }
    }
    result
}

fn read_input() -> Result<Vec<String>, io::Error> {
    let file = File::open(INPUT_FILE_PATH)?;
    let reader = BufReader::new(file);
    let mut lines = Vec::new();

    for line in reader.lines() {
        lines.push(line?);
    }

    Ok(lines)
}
