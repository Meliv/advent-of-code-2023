use core::num;
use std::ops::Sub;

use regex::Regex;

static INPUT_FILE_PATH: &str = "src/inputs/day_5.txt";

pub fn run() -> usize {
    let input = std::fs::read_to_string(INPUT_FILE_PATH).unwrap();

    let seeds: Vec<ValueRange> = load_seeds(&input);
    let maps: Vec<Map> = load_maps(input);

    let result: usize = calculate(&maps, seeds);

    println!("Result: {}", result);
    result
}

fn calculate(maps: &Vec<Map>, seeds: Vec<ValueRange>) -> usize {
    let mut value_ranges = seeds;

    for map in maps {
        let mut new_ranges: Vec<ValueRange> = vec![]; // This needs moving up

        for map_range in &map.ranges {
            for value_range in &value_ranges {
                let overlap = calculate_overlap(map_range, value_range);

                // Doesn't work if the value range overlaps multiple map ranges
                // Val Range: |     [_________]      [________________]
                // Map Range: |  [____]               [_]  [___] [_______]
                // Need to be more clever
                // calculate_overlap() needs to return an overlap containing a
                // Vec<ValueRange> for ranges that don't overlap anything
                // and another for ones that do

                // in_overlap are values that have been calculated and don't need to be provessed again for this map
                new_ranges.extend(overlap.in_overlap);
            }
        }

        value_ranges = new_ranges;
    }

    value_ranges
        .iter()
        .min_by(|x, y| x.min.cmp(&y.min))
        .unwrap()
        .min
}

fn calculate_overlap(map_range: &MapRange, value_range: &ValueRange) -> Overlap {
    Overlap {
        in_overlap: vec![],
        out_overlap: vec![],
    }
}

fn load_seeds(input: &str) -> Vec<ValueRange> {
    let mut seeds: Vec<ValueRange> = vec![];
    let exp = Regex::new(r"(\d+)\s+(\d+)").unwrap();
    let line = input.lines().next().unwrap();

    for c in exp.captures_iter(line) {
        let min: usize = c.get(1).unwrap().as_str().parse().unwrap();
        let range: usize = c.get(2).unwrap().as_str().parse().unwrap();
        seeds.push(ValueRange { min,range });
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
        let mut map_values: Vec<MapRange> = vec![];

        for y in x {
            let values: Vec<usize> = num_exp
                .captures_iter(y)
                .map(|f| f.get(0).unwrap().as_str().parse().unwrap())
                .collect();

            if !values.is_empty() {
                map_values.push(MapRange {
                    destination_start: *values.first().unwrap(),
                    source_start: *values.get(1).unwrap(),
                    range: *values.get(2).unwrap(),
                });
            }
        }

        maps.push(Map { ranges: map_values });
    }
    maps
}

#[derive(Debug)]
struct Overlap {
    in_overlap: Vec<ValueRange>,
    out_overlap: Vec<ValueRange>,
}

#[derive(Debug)]
struct ValueRange {
    min: usize,
    range: usize,
}

#[derive(Debug)]
struct Map {
    ranges: Vec<MapRange>,
}

#[derive(Debug)]
struct MapRange {
    source_start: usize,
    destination_start: usize,
    range: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day5_part2_test() {
        assert_eq!(run(), 46);
    }
}
