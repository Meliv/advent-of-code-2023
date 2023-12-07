use core::num;
use itertools::Itertools;

use regex::Regex;

static INPUT_FILE_PATH: &str = "src/inputs/day_7.txt";

pub fn run() -> usize {
    let input = std::fs::read_to_string(INPUT_FILE_PATH).unwrap();

    let hands: Vec<Hand> = input.lines().map(process_hand).collect();

    let hands_grouped_by_type = hands
    .into_iter()
    .sorted_by_key(|x| x.highest_hand)
    .group_by(|h| h.highest_hand);

    let mut sorted_hands: Vec<Hand> = vec![];

    for group in &hands_grouped_by_type {
        let mut group_vec: Vec<Hand> = group.1.collect();
        group_vec.sort_by(|a, b| a.cards.cmp(&b.cards));
        sorted_hands.extend(group_vec);
    }

    let result: usize = sorted_hands
        .iter()
        .enumerate()
        .map(|(i, h)| h.bid * (i + 1))
        .sum();

    println!("Result {}", result);

    result
}

fn process_hand(line: &str) -> Hand {
    let split: Vec<&str> = line.split(' ').collect();

    let cards = String::from(*split.first().unwrap());

    Hand {
        highest_hand: get_highest_hand(&cards),
        cards: translate_cards(&cards),
        bid: split.get(1).unwrap().parse().unwrap(),
    }
}

fn translate_cards(cards: &str) -> String {
    cards
        .replace('A', "Z")
        .replace('K', "Y")
        .replace('Q', "X")
        .replace('J', "W")
        .replace('T', "V")
}

fn get_highest_hand(cards: &str) -> HandType {
    for c in cards.chars() {
        let filtered_hand = cards.replace(c, "").to_string();

        let matching_cards: Vec<char> = cards.chars().filter(|x| x == &c).collect();

        match matching_cards.len() {
            5 => return HandType::FiveKind,
            4 => return HandType::FourKind,
            3 => match get_highest_hand(&filtered_hand) {
                HandType::OnePair => return HandType::FullHouse,
                _ => return HandType::ThreeKind,
            },
            2 => match get_highest_hand(&filtered_hand) {
                HandType::ThreeKind => return HandType::FullHouse,
                HandType::OnePair => return HandType::TwoPair,
                _ => return HandType::OnePair,
            },
            _ => {}
        }
    }

    HandType::HighCard
}

#[derive(Debug)]
struct Hand {
    cards: String,
    highest_hand: HandType,
    bid: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeKind = 4,
    FullHouse = 5,
    FourKind = 6,
    FiveKind = 7,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day7_part2_test() {
        assert_eq!(run(), 5905);
    }
}
