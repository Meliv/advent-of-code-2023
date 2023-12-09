use core::num;
use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

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
    let split: Vec<&str> = line.split_whitespace().collect();

    let cards = String::from(*split.first().unwrap());

    Hand {
        highest_hand: get_highest_hand(&cards),
        cards: translate_cards(&cards),
        untranslated_cards: cards,
        bid: split.get(1).unwrap().parse().unwrap(),
    }
}

fn translate_cards(cards: &str) -> String {
    cards
        .replace('A', "Z")
        .replace('K', "Y")
        .replace('Q', "X")
        .replace('J', "0")
        .replace('T', "V")
}

fn get_highest_hand(cards: &str) -> HandType {
    let joker_count: usize = cards.chars().filter(|c| c == &'J').count();

    match joker_count {
        0 => get_hand_type(cards),
        _ => get_joker_hand_type(cards, joker_count),
    }
}

fn get_joker_hand_type(cards: &str, joker_count: usize) -> HandType {
    let x = cards.replace('J', "");
    let filtered_cards = x.as_str();
    let unique_chars_hashset: HashSet<char> = filtered_cards.chars().collect();
    let unique_chars = unique_chars_hashset.len();

    match joker_count {
        5 => HandType::FiveKind,
        4 => HandType::FiveKind,
        // 3 cards could be Full House (JJJAA), FourKind (JJJA2)
        3 => match unique_chars {
            1 => HandType::FiveKind,
            _ => HandType::FourKind,
        },
        // 2 cards could be Five Kind (JJAAA), Two Pair (JJA22), Three Kind (JJA23)
        2 => match unique_chars {
            1 => HandType::FiveKind,
            2 => HandType::FourKind,
            _ => HandType::ThreeKind,
        },
        // J3JAA
        // 1 card could be FiveKind (JAAAA), FourKind (JA222) FullHouse (JAA22), ThreeKind (JA233), Pair (JA234)
        1 => match unique_chars {
            1 => HandType::FiveKind,
            2 => {
                let mut char_counts: HashMap<char, usize> = HashMap::new(); // ChatGPT written all over this
                for ch in filtered_cards.chars() {
                    *char_counts.entry(ch).or_insert(0) += 1;
                }

                let f = char_counts.iter().next().unwrap();
                match f.1 {
                    2 => HandType::FullHouse,
                    _ => HandType::FourKind,
                }
            }
            3 => HandType::ThreeKind,
            _ => HandType::OnePair,
        },
        _ => HandType::HighCard, // Never hit
    }
}

fn get_hand_type(cards: &str) -> HandType {
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
    untranslated_cards: String,
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
        assert_eq!(run(), 248747492);
    }
}
