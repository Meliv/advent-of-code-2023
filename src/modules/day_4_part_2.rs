use regex::Regex;

static INPUT_FILE_PATH: &str = "src/inputs/day_4.txt";

pub fn run() -> u32 {
    let master_list = std::fs::read_to_string(INPUT_FILE_PATH).unwrap();
    let mut result: u32 = master_list.lines().count() as u32;
    println!("Total: {}", result);

    let mut cards_to_process: Vec<&str> = master_list.lines().collect();

    while !cards_to_process.is_empty() {



    }

    //cards_to_process.extend(master_list.lines().skip(i+1).take(wins));


    println!("Card To Process {:?}", cards_to_process);

    println!("Total: {}", result);
    result
}

fn process_card(card: &str) -> usize {
    let game_split: Vec<&str> = card.split(':').collect();
    
    let game_without_line = game_split.first().unwrap();
    let line_without_game = game_split.last().unwrap();

    let splits: Vec<&str> = line_without_game.split('|').collect();
    let win_nos: Vec<&str> = splits.first().unwrap().trim().split(' ').filter(|c| !c.is_empty()).collect();
    let your_nos: Vec<&str> = splits.get(1).unwrap().trim().split(' ').filter(|c| !c.is_empty()).collect();

    win_nos.iter().filter(|n| your_nos.contains(n)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day4_part1_test() {
        assert_eq!(run(), 30);
    }
}
