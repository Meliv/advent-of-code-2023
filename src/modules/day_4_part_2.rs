use regex::Regex;

static INPUT_FILE_PATH: &str = "src/inputs/day_4.txt";

pub fn run() -> u32 {
    let lines = std::fs::read_to_string(INPUT_FILE_PATH).unwrap();
    let games = get_games(lines);

    let mut games_to_process = games.to_vec();
    let mut total_games: u32 = 0;

    loop {
        let mut new_games: Vec<Game> = vec![];
        total_games += games_to_process.len() as u32;

        for card in games_to_process.iter() {
            if card.wins != 0 {
                let s: usize = card.game_no;
                let e: usize = s + card.wins;

                new_games.extend(games[s..e].to_vec());
            }
        }
        if new_games.is_empty() {
            break;
        } else {
            games_to_process = new_games;
        }
    }

    println!("Total Games: {total_games}");

    total_games
}

fn get_games(cards: String) -> Vec<Game> {
    let mut games: Vec<Game> = vec![];

    for card in cards.lines() {
        games.push(Game::new(card));
    }
    games
}

#[derive(Debug, Clone)]
struct Game {
    game_no: usize,
    wins: usize,
}

impl Game {
    fn new(card: &str) -> Game {
        let game_split: Vec<&str> = card.split(':').collect();

        let game_no: usize = game_split
            .first()
            .unwrap()
            .replace("Card", "")
            .trim()
            .parse()
            .unwrap();

        let nos_split: Vec<&str> = game_split.last().unwrap().split(" | ").collect();

        let win_nos_str: Vec<&str> = nos_split
            .first()
            .unwrap()
            .split_whitespace()
            .collect();

        let your_nos_str: Vec<&str> = nos_split
            .last()
            .unwrap()
            .split_whitespace()
            .collect();

        let your_nos: Vec<u32> = your_nos_str.iter().map(|n| n.parse().unwrap()).collect();
        let win_nos: Vec<u32> = win_nos_str.iter().map(|n| n.parse().unwrap()).collect();
        let wins = your_nos.iter().filter(|n| win_nos.contains(n)).count();

        Game {
            game_no,
            wins,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day4_part2_test() {
        assert_eq!(run(), 5625994);
    }
}
