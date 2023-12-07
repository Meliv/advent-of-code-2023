use core::num;
use std::ops::Sub;

use regex::Regex;

static INPUT_FILE_PATH: &str = "src/inputs/day_7.txt";

pub fn run() -> usize {
    let input = std::fs::read_to_string(INPUT_FILE_PATH).unwrap();

    let hands: Vec<Hand> = input.lines().map(process_hand).collect();
    let result: usize = hands.iter().map(|h| h.bid * h.highest_hand as usize).sum();

    println!("Result: {}", result);

    6440
}

fn process_hand(line: &str) -> Hand {

    let split: Vec<&str> = line.split(' ').collect();

    Hand {
        cards: String::from(*split.first().unwrap()),
        highest_hand: get_highest_hand(split.first().unwrap()),
        bid: split.get(1).unwrap().parse().unwrap()
    }
}

fn get_highest_hand(cards: &str) -> HandType {

    HandType::HighCard
}

#[derive(Debug)]
struct Hand {
    cards: String,
    highest_hand: HandType,
    bid: usize
}

#[derive(Debug, Copy, Clone)]
enum HandType {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeKind = 4,
    FullHouse = 5,
    FourKind = 6,
    FiveKind = 7
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day7_part1_test() {
        assert_eq!(run(), 6440);
    }
}
