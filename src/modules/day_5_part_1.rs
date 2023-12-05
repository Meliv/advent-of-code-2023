use regex::Regex;

static INPUT_FILE_PATH: &str = "src/inputs/day_5.txt";

pub fn run() -> usize {
    let input = std::fs::read_to_string(INPUT_FILE_PATH).unwrap();

    let seeds: Vec<usize> = load_seeds(&input);

    println!("{:?}", seeds);

    //let maps = load_maps(input);

    0
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

    for line in input.lines() {
        maps.push(Map::new(&line));
    }

    maps
}

#[derive(Debug)]
struct Map {
    values: Vec<MapValue>,
}

impl Map {
    fn new(line: &&str) -> Map {
        Map {
            values: vec![MapValue::new(line)],
        }
    }
}

#[derive(Debug)]
struct MapValue {
    //source_start: usize,
    //destination_start: usize,
    //range: usize
    value: String,
}

impl MapValue {
    fn new(line: &str) -> MapValue {
        MapValue {
            value: line.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day5_part1_test() {
        assert_eq!(run(), 35);
    }
}
