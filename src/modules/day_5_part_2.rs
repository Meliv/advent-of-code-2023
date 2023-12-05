use core::num;

use regex::Regex;

static INPUT_FILE_PATH: &str = "src/inputs/day_5.txt";

pub fn run() -> usize {
    let input = std::fs::read_to_string(INPUT_FILE_PATH).unwrap();

    let seeds: Vec<usize> = load_seeds(&input);
    let maps = load_maps(input);

    let result: usize = seeds.iter().map(|s| get_location(&maps, *s)).min().unwrap();

    println!("Result: {}", result);

    result
}

fn get_location(maps: &Vec<Map>, seed: usize) -> usize {
    let mut value = seed;

    for map in maps {
        let x: &Vec<usize> = &map
            .values
            .iter()
            .filter(|v| value >= v.source_start && value <= v.source_end)
            .map(|v| value - v.source_start + v.destination_start)
            .collect();

        value = match x.first() {
            Some(v) => *v,
            None => value
        };
    }

    value
}

fn load_seeds(input: &str) -> Vec<usize> {
    let mut seeds: Vec<usize> = vec![];
    let exp = Regex::new(r"\d+").unwrap();
    let line = input.lines().next().unwrap();

    for c in exp.captures_iter(line) {
        seeds.push(c.get(0).unwrap().as_str().parse().unwrap());
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
                    source_start: *values.get(1).unwrap(),
                    source_end: *values.get(1).unwrap() + *values.get(2).unwrap() - 1
                });
            }
        }

        maps.push(Map { values: map_values });
    }

    maps
}

#[derive(Debug)]
struct Map {
    values: Vec<MapValue>,
}

#[derive(Debug)]
struct MapValue {
    source_start: usize,
    source_end: usize,
    destination_start: usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day5_part1_test() {
        assert_eq!(run(), 88151870);
    }
}
