use core::num;
use std::ops::Sub;

use regex::Regex;

static INPUT_FILE_PATH: &str = "src/inputs/day_5.txt";

pub fn run() -> usize {
    let input = std::fs::read_to_string(INPUT_FILE_PATH).unwrap();

    let seeds: Vec<Seed> = load_seeds(&input);
    let reversed_maps: Vec<Map> = load_maps(input);

    println!("{:?}", reversed_maps);

    if seeds.len() != 0 {
        //panic!();

    }

    // Figure out min output for first map
    let mut last_map = reversed_maps.first().unwrap();
    let mut min_value: usize = last_map
    .values
    .iter()
    .min_by(|x,y| x.destination_start.cmp(&y.destination_start)).unwrap()
    .destination_start;

    min_value = min_value.min(0);

    // From there, iterate until you get a seed that's in your seed ranges
    loop {
        let result = get_seed(&reversed_maps, min_value);
        if result.is_some() && is_in_seeds(&seeds, min_value) { break }
        min_value += 1;
    }

    // First one you get, return it
    println!("Result: {}", min_value);
    min_value
}

fn is_in_seeds(seeds: &Vec<Seed>, value: usize) -> bool {
    seeds.iter().any(|s| s.min <= value && s.max >= value)
}

fn get_seed(maps: &Vec<Map>, location: usize) -> Option<usize> {
    let mut value = location;

    println!("Huh? {:?}", location);
    for map in maps {
        let x: &Vec<usize> = &map
            .values
            .iter()
            //.filter(|v| value >= v.destination_start && value <= v.destination_end)
            .map(|v| value - v.destination_start + v.source_start)
            .collect();

            if x.first().is_some() {
                println!("{}", x.first().unwrap());
            }


        value = match x.first() {
            Some(v) => *v,
            None => return None
        };
    }

    Some(value)
}

fn load_seeds(input: &str) -> Vec<Seed> {
    let mut seeds: Vec<Seed> = vec![];
    let exp = Regex::new(r"(\d+)\s+(\d+)").unwrap();
    let line = input.lines().next().unwrap();

    for c in exp.captures_iter(line) {
        let min: usize = c.get(1).unwrap().as_str().parse().unwrap();
        let range: usize = c.get(2).unwrap().as_str().parse().unwrap();
        let max = range + min;
        seeds.push(Seed { min, max });
    }

    seeds
}

fn load_maps(input: String) -> Vec<Map> {
    let mut maps: Vec<Map> = vec![];

    let split_exp = Regex::new(r"\n\w+-\w+-\w+\smap:").unwrap();
    let num_exp = Regex::new(r"\d+").unwrap();

    let blocks: Vec<_> = split_exp.split(&input).collect();

    let bc: Vec<&str> = blocks[1..].to_vec();

    for b in bc {
        let x: Vec<_> = b.split('\n').collect();
        let mut map_values: Vec<MapValue> = vec![];

        for y in x {
            let values: Vec<usize> = num_exp
                .captures_iter(y)
                .map(|f| f.get(0).unwrap().as_str().parse().unwrap())
                .collect();

            if !values.is_empty() {
                map_values.push(MapValue {
                    destination_start: *values.first().unwrap(),
                    destination_end: *values.first().unwrap() + *values.get(2).unwrap() - 1,
                    source_start: *values.get(1).unwrap(),
                    source_end: *values.get(1).unwrap() + *values.get(2).unwrap() - 1,
                    offset: values.first().unwrap().abs_diff(*values.get(1).unwrap())
                });
            }
        }

        maps.push(Map { values: map_values });
    }

    maps.reverse();
    maps
}

#[derive(Debug)]
struct Seed {
    max: usize,
    min: usize
}

#[derive(Debug)]
struct Map {
    values: Vec<MapValue>,
}

#[derive(Debug)]
struct MapValue {
    source_start: usize,
    source_end: usize,
    destination_start: usize,
    destination_end: usize,
    offset: usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day5_part1_test() {
        assert_eq!(run(), 88151870);
    }
}
