use num::{
    clamp,
    traits::{clamp_max, clamp_min},
};
use regex::Regex;

static INPUT_FILE_PATH: &str = "src/inputs/day_3.txt";

static REGEX_GET_DIGITS: &str = r"\d+";

pub fn run() -> isize {
    let mut input = std::fs::read_to_string(INPUT_FILE_PATH).unwrap();
    let line_length: isize = input.lines().next().unwrap().len() as isize;
    input = input.replace(['\r', '\n'], "");
    let digit_exp = Regex::new(REGEX_GET_DIGITS).unwrap();
    let mut result: isize = 0;

    for capture in digit_exp.captures_iter(input.as_str()) {
        let match_start: isize = capture.get(0).unwrap().start() as isize;
        let match_end: isize = capture.get(0).unwrap().end() as isize;

        if has_adjacent_symbols(match_start, match_end, line_length, &input) {
            let number: isize = capture.get(0).unwrap().as_str().parse().unwrap();
            result += number;
        }
    }
    println!("Total: {result}");

    result
}

fn has_adjacent_symbols(start: isize, end: isize, line_length: isize, input: &str) -> bool {
    let line_no = (start / line_length) + 1;
    let line_start = (line_no - 1) * line_length;
    let line_end = line_no * line_length - 1;

    // Above
    if line_no != 1 {
        let above_match_start = clamp_min(start - line_length - 1, line_start - line_length);
        let above_match_end = clamp_max(end - line_length + 1, line_end - line_length);

        let ams_u: usize = above_match_start as usize;
        let ame_u: usize = above_match_end as usize;

        let chars_above = &input[ams_u..ame_u];

        if chars_above.chars().any(|c| !c.is_numeric() && c != '.') {
            return true;
        }
    }

    let adjacent_match_start = clamp_min(start - 1, line_start) as usize;
    let adjacent_match_end = clamp_max(end + 1, line_end) as usize;

    let adms_u: usize = adjacent_match_start as usize;
    let adme_u: usize = adjacent_match_end as usize;

    let chars_adjacent = &input[adms_u..adme_u];
    if chars_adjacent.chars().any(|c| !c.is_numeric() && c != '.') {
        return true;
    }

    if line_no != (input.len() / line_length as usize) as isize {
        let adjacent_match_start = clamp_min(start + line_length - 1, line_start + line_length);
        let adjacent_match_end = clamp_max(end + line_length + 1, line_end + line_length);

        let bms_u: usize = adjacent_match_start as usize;
        let bme_u: usize = adjacent_match_end as usize;

        let below_adjacent = &input[bms_u..bme_u];
        if below_adjacent.chars().any(|c| !c.is_numeric() && c != '.') {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day3_part1_test() {
        assert_eq!(run(), 553825);
    }
}
