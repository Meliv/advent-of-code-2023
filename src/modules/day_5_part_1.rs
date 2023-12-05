use core::num;

use regex::Regex;

static INPUT_FILE_PATH: &str = "src/inputs/day_5.txt";

pub fn run() -> usize {
    let input = std::fs::read_to_string(INPUT_FILE_PATH).unwrap();

    let seeds: Vec<usize> = load_seeds(&input);
    let maps = load_maps(input);

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

    let split_exp = Regex::new(r"\n\w+-\w+-\w+\smap:").unwrap();
    let num_exp = Regex::new(r"\d+").unwrap();

    let blocks: Vec<_> = split_exp.split(&input).collect();

    let bc: Vec<&str> = blocks[1..].to_vec();

    for b in bc {
        //println!("block start");

        let x: Vec<_> = b.split('\n').collect();

        let mut map_values: Vec<MapValue> = vec![];

        for y in x {
            // Declare map value here

            let values: Vec<usize> = num_exp
                .captures_iter(y)
                .map(|f| f.get(0).unwrap().as_str().parse().unwrap())
                .collect();

            if !values.is_empty() {
                // Hack
                map_values.push(MapValue {
                    destination_start: *values.first().unwrap(),
                    source_start: *values.get(1).unwrap(),
                    range: *values.get(2).unwrap(),
                });
            }

            //println!("line break");
        }

        maps.push(Map { values: map_values });

        //println!("block end");
    }

    for (i, map) in maps.iter().enumerate() {
        
        println!("Map {}", i+1);

        let x = &map.values;

        for mapvalue in x {

            println!("{:?}", mapvalue);

        }
    }


    /*
    for b in blocks[1..blocks.len()-1] {
        println!("{}", b);

    }
    */

    /*
    for line in input.lines() {
        maps.push(Map::new(&line));
    }
    */

    maps
}

#[derive(Debug)]
struct Map {
    values: Vec<MapValue>,
}

#[derive(Debug)]
struct MapValue {
    source_start: usize,
    destination_start: usize,
    range: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day5_part1_test() {
        assert_eq!(run(), 35);
    }
}
