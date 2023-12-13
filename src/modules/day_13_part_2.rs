use itertools::{self, Itertools};
use regex::Regex;
use std::{collections::HashSet, str::FromStr};

const INPUT_FILE_PATH: &str = "src/inputs/day_13.txt";

pub fn run() -> usize {
    let input = std::fs::read_to_string(INPUT_FILE_PATH).unwrap();
    let mut result = 0;

    let fields: Vec<String> = input
        .split_terminator("\r\n\r\n")
        .map(|s| s.to_string())
        .collect();

    for (i, field) in fields.iter().enumerate() {
        let r = horizontal(field);

        match r {
            Some(r) => result += r * 100,
            None => match vertical(field) {
                Some(v) => result += v,
                None => {
                    println!("Failed on input {}", i + 1);
                    panic!()
                }
            },
        }
    }

    println!("Result {}", result);
    result
}

fn vertical(input: &str) -> Option<usize> {
    let mut rotated_input_chars: Vec<Vec<char>> = vec![];

    let mut i = 0;
    while i < input.lines().next().unwrap().len() {
        let mut new_line: Vec<char> = vec![];
        for line in input.lines() {
            let c = line.chars().nth(i).unwrap();
            new_line.push(c);
        }
        let x: Vec<char> = new_line.iter().rev().copied().collect::<Vec<char>>();
        rotated_input_chars.push(x);
        i += 1;
    }

    let rotated_input: String = rotated_input_chars
        .iter()
        .map(|inner_vec| inner_vec.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");

    horizontal(&rotated_input)
}

fn horizontal(input: &str) -> Option<usize> {
    for (a, b) in input.lines().enumerate().tuple_windows() {
        if a.1 == b.1 || smudge_compare(a.1, b.1) {
            let backtrace = back_trace(input, a.0, b.0);
            if backtrace.is_some() {
                return backtrace;
            }
        }
    }

    None
}

fn back_trace(input: &str, a: usize, b: usize) -> Option<usize> {
    let mut a_i = a as isize;
    let mut b_i = b;

    let mut smudge_compare_count = 0;
    while a_i >= 0 && b_i < input.lines().count() {
        let a_s = input.lines().nth(a_i as usize).unwrap();
        let b_s = input.lines().nth(b_i).unwrap();

        if a_s != b_s {
            match smudge_compare(a_s, b_s) {
                true => smudge_compare_count += 1,
                false => return None
            };
        }

        a_i -= 1;
        b_i += 1;
    }

    if smudge_compare_count == 1 {
        return Some(b);
    }

    None
}

fn smudge_compare(a: &str, b: &str) -> bool {
    let mut a_chars: Vec<char> = a.chars().collect_vec();
    let b_chars: Vec<char> = b.chars().collect_vec();

    let mut i = 0;
    while i < a.len() {
        a_chars[i] = flip_char(a_chars[i]);

        if a_chars == b_chars {
            return true;
        }

        a_chars[i] = flip_char(a_chars[i]);

        i += 1;
    }

    false
}

fn flip_char(c: char) -> char {
    match c {
        '#' => '.',
        '.' => '#',
        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day13_part2_test() {
        assert_eq!(run(), 30449);
    }
}
