use regex::Regex;

static INPUT_FILE_PATH: &str = "src/inputs/day_4.txt";

pub fn run() -> u32 {
    let input = std::fs::read_to_string(INPUT_FILE_PATH).unwrap();
    let mut result: u32 = 0;

    for current_line in input.lines() {
        let mut line_points: u32 = 0;
        let line_without_game = current_line.split(':').nth(1).unwrap();
        let splits: Vec<&str> = line_without_game.split("|").collect();
        let win_nos: Vec<&str> = splits.get(0).unwrap().trim().split(' ').filter(|c| !c.is_empty()).collect();
        let your_nos: Vec<&str> = splits.get(1).unwrap().trim().split(' ').filter(|c| !c.is_empty()).collect();

        for win_no in win_nos {
            if your_nos.contains(&win_no) {
                if line_points == 0 {
                    line_points += 1
                } else {
                    line_points *= 2
                }
            }
        }
        
        result += line_points;
    }

    println!("Total: {}", result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day4_part1_test() {
        assert_eq!(run(), 22193);
    }
}
