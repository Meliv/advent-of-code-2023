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
    let mut next_map_uncalculated_value_ranges: Vec<ValueRange> = seeds;

    for map in maps {
        let mut this_map_calculated_ranges: Vec<ValueRange> = vec![];

        // A counter for the loop
        // We can't use .iter().enumerate() here as it'll transform
        // value_range: ValueRange
        // into
        // value_range: &ValueRange
        // which prevents us from updating the min/range properties as we go along
        let mut i: usize = 1;
        let next_map_uncalculated_value_ranges_size: usize =
            next_map_uncalculated_value_ranges.len();

        for mut value_range in next_map_uncalculated_value_ranges {
            for map_range in &map.ranges {
                let overlap = calculate_overlap(map_range, &value_range);

                // Doesn't work if the value range overlaps multiple map ranges
                // Val Range: |     [_________]      [________________]
                // Map Range: |  [____]               [_]  [___] [_______]
                // Need to be more clever
                // calculate_overlap() needs to return an overlap containing a
                // Vec<ValueRange> for ranges that don't overlap anything
                // and another for ones that do

                // in_overlap are values that have been calculated and don't need to be processed again for this map
                if let Some(in_overlap) = overlap.in_overlap {
                    this_map_calculated_ranges.push(in_overlap);
                }

                if let Some(out_overlap) = overlap.out_overlap {
                    value_range.min = out_overlap.min;
                    value_range.range = out_overlap.range;

                    if i == next_map_uncalculated_value_ranges_size {
                        // Last iteration so whatever we're left over
                        // with in value_range needs to be saved
                        this_map_calculated_ranges.push(out_overlap);
                    }
                }

                i += 1;
            }
        }

        next_map_uncalculated_value_ranges = this_map_calculated_ranges;
    }

    next_map_uncalculated_value_ranges
        .iter()
        .min_by(|x, y| x.min.cmp(&y.min))
        .unwrap()
        .min
}

fn calculate_overlap(map_range: &MapRange, value_range: &ValueRange) -> Overlap {
    if value_range.min > map_range.source_end || value_range.max < map_range.source_start {
        // No Overlap
        return Overlap {
            in_overlap: None,
            out_overlap: Some(ValueRange {
                min: value_range.min,
                max: value_range.max,
                range: value_range.range,
            }),
        };
    } else if value_range.min >= map_range.source_start && value_range.max <= map_range.source_end {
        // Entire Overlap
        return Overlap {
            in_overlap: Some(ValueRange {
                min: (value_range.min - map_range.source_start) + map_range.destination_start,
                max: (value_range.max - map_range.source_start) + map_range.destination_start,
                range: value_range.range,
            }),
            out_overlap: None,
        };
    } else if value_range.min >= map_range.source_start && value_range.max > map_range.source_end {
        // Partial Overlap Right
        // Value Range:     [___|____]
        // Map Range  : [_______]
        Overlap {
            in_overlap: Some(ValueRange {
                min: (value_range.min - map_range.source_start) + map_range.destination_start,
                max: map_range.destination_start + map_range.range - 1,
                range: map_range.source_end - value_range.min + 1
            }),
            out_overlap: Some(ValueRange {
                min: map_range.source_end + 1,
                max: map_range.source_end + (value_range.range - (map_range.source_end - value_range.min + 1)),
                range: value_range.range - (map_range.source_end - value_range.min + 1)
            })
        }
    }
    else {
        // Partial Overlap Left
        // Value Range: [________]
        // Map Range  :      [_______] 
        Overlap {
            in_overlap: Some(ValueRange {
                min: value_range.min,
                max: value_range.max,
                range: value_range.range,
            }),
            out_overlap: None,
        }
    }
}

fn load_seeds(input: &str) -> Vec<ValueRange> {
    let mut seeds: Vec<ValueRange> = vec![];
    let exp = Regex::new(r"(\d+)\s+(\d+)").unwrap();
    let line = input.lines().next().unwrap();

    for c in exp.captures_iter(line) {
        let min: usize = c.get(1).unwrap().as_str().parse().unwrap();
        let range: usize = c.get(2).unwrap().as_str().parse().unwrap();
        let max: usize = min + range - 1;
        seeds.push(ValueRange { min, range, max });
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
                    source_end: *values.get(1).unwrap() + *values.get(2).unwrap() - 1,
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
    in_overlap: Option<ValueRange>,
    out_overlap: Option<ValueRange>,
}

#[derive(Debug)]
struct ValueRange {
    min: usize,
    max: usize,
    range: usize,
}

#[derive(Debug)]
struct Map {
    ranges: Vec<MapRange>,
}

#[derive(Debug)]
struct MapRange {
    source_start: usize,
    source_end: usize,
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
