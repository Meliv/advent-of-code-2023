use core::{num, panic};

use regex::Regex;

static INPUT_FILE_PATH: &str = "src/inputs/day_5.txt";

pub fn run() -> usize {
    let input = std::fs::read_to_string(INPUT_FILE_PATH).unwrap();

    let seeds: Vec<usize> = load_seeds(&input);
    let maps = load_maps(input);

    let result = get_location(&maps, seeds);
    let min_result = result.iter().min().unwrap();

    println!("Result: {}", min_result);

    *min_result
}

fn get_location(maps: &Vec<Map>, seeds: Vec<usize>) -> Vec<usize> {

    let mut successful_seeds: Vec<usize> = vec![];

    for mut value in seeds {
        
        for map in maps {
            let x: &Vec<usize> = &map
                .values
                .iter()
                .filter(|v| value >= v.source_start && value <= v.source_end)
                .map(|v| value - v.source_start + v.destination_start)
                .collect();

            value = *x.first().unwrap_or(&value);
        }

        successful_seeds.push(value);
    }

    successful_seeds
}

fn load_seeds(input: &str) -> Vec<usize> {
    let mut seeds: Vec<usize> = vec![];
    let exp = Regex::new(r"(\d+)\s+(\d+)").unwrap();
    let line = input.lines().next().unwrap();

    for c in exp.captures_iter(line) {
        let s: usize = c.get(1).unwrap().as_str().parse().unwrap();
        let e: usize = c.get(2).unwrap().as_str().parse().unwrap();

        let a: Vec<usize> = (s..s + e).collect();
        seeds.extend(a);
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
                });
            }
        }

        let x = map_values
            .iter()
            .min_by(|x, y| x.destination_start.cmp(&y.destination_start))
            .unwrap();

        maps.push(Map {
            min_value: x.source_start,
            max_value: x.source_end,
            values: map_values,
        });
    }

    maps
}

#[derive(Debug)]
struct Map {
    values: Vec<MapValue>,
    min_value: usize,
    max_value: usize,
}

#[derive(Debug)]
struct MapValue {
    source_start: usize,
    source_end: usize,
    destination_start: usize,
    destination_end: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day5_part1_test() {
        assert_eq!(run(), 88151870);
    }
}
