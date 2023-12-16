use std::collections::VecDeque;

use itertools::Itertools;

const INPUT_FILE_PATH: &str = "src/inputs/day_15.txt";

pub fn run() -> usize {
    let input = std::fs::read_to_string(INPUT_FILE_PATH).unwrap();
    let x: String = input.replace(['\r', '\n'], "");
    let replaced_input: Vec<&str> = x.split(',').collect();

    let mut map: Map = Map::new();
    for v in replaced_input {
        map.process(v);
    }

    let result = map.get_result();

    println!("Result: {}", result);
    result
}

#[derive(Debug)]
struct Map {
    boxes: Vec<LensBox>,
}

impl Map {
    fn new() -> Map {
        let mut boxes: Vec<LensBox> = Vec::with_capacity(256);
        for _ in 0..256 {
            boxes.push(LensBox::new());
        }
        Map { boxes }
    }

    fn get_result(&self) -> usize {
        self.boxes
            .iter()
            .enumerate()
            .map(|(i, b)| (i + 1) * b.get_result())
            .sum()
    }

    fn process(&mut self, value: &str) {
        match value.contains('=') {
            true => self.add_lens(value),
            _ => self.remove_lens(value),
        };
    }

    fn add_lens(&mut self, value: &str) {
        let split_values: Vec<&str> = value.split('=').collect();
        let label = *split_values.first().unwrap();
        let lens_value: usize = split_values.get(1).unwrap().parse().unwrap();
        let box_index = self.calculate_hash(label);

        self.boxes
            .get_mut(box_index)
            .unwrap()
            .add_lens(label, lens_value);
    }

    fn remove_lens(&mut self, value: &str) {
        let label: &str = &value[0..value.len() - 1];
        let box_index = self.calculate_hash(label);

        self.boxes.get_mut(box_index).unwrap().remove_lens(label);
    }

    fn calculate_hash(&self, input: &str) -> usize {
        let mut result = 0;
        for c in input.chars() {
            result += c as usize;
            result *= 17;
            result %= 256;
        }
        result
    }
}

#[derive(Debug)]
struct LensBox {
    lenses: VecDeque<(String, usize)>,
}

impl LensBox {
    fn new() -> LensBox {
        LensBox {
            lenses: VecDeque::new(),
        }
    }

    fn add_lens(&mut self, label: &str, value: usize) {
        let lens = self.lenses.iter_mut().find(|l| l.0 == label);
        match lens {
            Some(l) => l.1 = value,
            None => self.lenses.push_back((label.to_string(), value)),
        }
    }

    fn remove_lens(&mut self, label: &str) {
        let lens = self.lenses.iter().find_position(|l| l.0 == label);

        if let Some(l) = lens {
            self.lenses.remove(l.0);
        }
    }

    fn get_result(&self) -> usize {
        let mut result = 0;
        for (i, lens) in self.lenses.iter().enumerate() {
            result += (1 + i) * lens.1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day15_part2_test() {
        assert_eq!(run(), 212449);
    }
}
