use regex::Regex;

static INPUT_FILE_PATH: &str = "src/inputs/day_3.txt";

static REGEX_GET_DIGITS: &str = r"\d+";
static REGEX_GET_SYMBOLS: &str = r"[^\d.\n]";

pub fn run() {
    let input = std::fs::read_to_string(INPUT_FILE_PATH).unwrap();
    let digit_exp = Regex::new(REGEX_GET_DIGITS).unwrap();
    let mut result: u32 = 0;

    for (i, current_line) in input.lines().enumerate() {
        let prev_line = if i != 0 {
            input.lines().nth(i - 1).unwrap()
        } else {
            ""
        };
        let next_line = if i != input.lines().count() - 1 {
            input.lines().nth(i + 1).unwrap()
        } else {
            ""
        };

        for captures in digit_exp.captures_iter(current_line) {
            let match_start = captures.get(0).unwrap().start();
            let match_end = captures.get(0).unwrap().end();

            let adjacent_prev = has_adjacent_symbols(match_start, match_end, prev_line);
            let adjacent_next = has_adjacent_symbols(match_start, match_end, next_line);
            let adjacent_current = has_adjacent_symbols(match_start, match_end, current_line);

            if adjacent_next || adjacent_prev || adjacent_current {
                let r: u32 = captures.get(0).unwrap().as_str().parse().unwrap();
                result += r;
            }
        }
    }
    println!("Total: {result}");
}

fn has_adjacent_symbols(start: usize, end: usize, line: &str) -> bool {
    if line.is_empty() {
        return false;
    }
    let match_start = if start > 0 { start - 1 } else { 0 };
    let match_end = if end == line.len() { end } else { end + 1 };

    let symbol_exp = Regex::new(REGEX_GET_SYMBOLS).unwrap();

    symbol_exp.find(&line[match_start..match_end]).is_some()
}
