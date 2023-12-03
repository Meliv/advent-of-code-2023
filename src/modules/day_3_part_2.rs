use regex::Regex;

static INPUT_FILE_PATH: &str = "src/inputs/day_3.txt";

static REGEX_GET_NUMBERS: &str = r"\b\d+";
static REGEX_GET_STARS: &str = r"\*+";

pub fn run() -> u32 {
    let input = std::fs::read_to_string(INPUT_FILE_PATH).unwrap();
    let star_exp = Regex::new(REGEX_GET_STARS).unwrap();
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

        for captures in star_exp.captures_iter(current_line) {
            let match_start = captures.get(0).unwrap().start() - 1;
            let match_end = captures.get(0).unwrap().end();

            let mut adjacent_numbers: Vec<u32> = vec![];
            adjacent_numbers.extend(get_adjacent_numbers(match_start, match_end, prev_line));
            adjacent_numbers.extend(get_adjacent_numbers(match_start, match_end, next_line));
            adjacent_numbers.extend(get_adjacent_numbers(match_start, match_end, current_line));

            if adjacent_numbers.len() == 2 {
                result += adjacent_numbers.first().unwrap() * adjacent_numbers.last().unwrap();
            }
        }
    }
    println!("Total: {result}");

    result
}

fn get_adjacent_numbers(start: usize, end: usize, line: &str) -> Vec<u32> {
    if line.is_empty() {
        return vec![];
    }

    Regex::new(REGEX_GET_NUMBERS)
    .unwrap()
    .captures_iter(line)
    .filter(|c| c.get(0).unwrap().start() <= end && c.get(0).unwrap().end() > start)
    .map(|c|c.get(0).unwrap().as_str().parse().unwrap())
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day3_part2_test() {
        assert_eq!(run(), 93994191);
    }
}