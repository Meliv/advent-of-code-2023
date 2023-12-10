use std::thread::current;

use regex::Regex;

const INPUT_FILE_PATH: &str = "src/inputs/day_10.txt";
const START_REGEX: &str = "S";

pub fn run() -> usize {
    let mut input = std::fs::read_to_string(INPUT_FILE_PATH).unwrap();
    let line_length: isize = input.lines().next().unwrap().len() as isize;
    input = input.replace(['\r', '\n'], "");
    let exp = Regex::new(START_REGEX).unwrap();
    let start = exp.find(&input).unwrap();

    let mut result = 1;
    let start_direction = get_starting_direction(&input, start.start(), line_length as usize);

    // Mutable in the loop for pathfinding
    let mut current_tile: char = start_direction.0;
    let mut current_pos: isize = start_direction.1 as isize;
    let mut last_pos: isize = start.start() as isize;
    let mut next_pos: isize;

    // Friendly names for movement directions
    let south: isize = line_length;
    let north: isize = -line_length;
    let west: isize = -1;
    let east: isize = 1;

    while current_tile != 'S' {

        // Debug
        println!("Step {}: At {} with {}. Last pos {}", result, current_pos, current_tile, last_pos);
        
        next_pos = match current_tile {
            '|' => {
                // North/South
                match current_pos > last_pos {
                    true => south,
                    false => north,
                    _ => panic!(),
                }
            }
            '-' => {
                // West/East
                match current_pos > last_pos {
                    true => east,
                    false => west,
                    _ => panic!(),
                }
            }
            'L' => {
                // North/East
                match current_pos < last_pos {
                    true => north,
                    false => east,
                    _ => panic!(),
                }
            }
            'J' => {
                // North/West
                match current_pos - 1 > last_pos {
                    true => west,
                    false => north,
                    _ => panic!(),
                }
            }
            '7' => {
                // South/West
                match current_pos > last_pos {
                    true => south,
                    false => west,
                    _ => panic!(),
                }
            }
            'F' => {
                //South/East
                match current_pos + 1 < last_pos {
                    true => east,
                    false => south,
                    _ => panic!(),
                }
            }
            _ => panic!("Tile: {}, Pos {}", current_tile, current_pos),
        };

        last_pos = current_pos;
        current_pos += next_pos;
        current_tile = input.chars().nth(current_pos as usize).unwrap();
        result += 1;
    }

    println!("Result {}", result / 2);

    result / 2
}

fn get_starting_direction(input: &str, position: usize, line_length: usize) -> (char, usize) {
    let valid_west_tiles = ['-', 'L', 'F'];
    let west = input.chars().nth(position - 1).unwrap();
    if valid_west_tiles.contains(&west) {
        return (west, position - 1);
    }

    let valid_east_tiles = ['-', 'J', '7'];
    let east = input.chars().nth(position + 1).unwrap();
    if valid_east_tiles.contains(&east) {
        return (east, position + 1);
    }

    let valid_north_tiles = ['|', 'J', 'F'];
    let north = input.chars().nth(position - line_length).unwrap();
    if valid_north_tiles.contains(&north) {
        return (north, position - line_length);
    }

    let valid_south_tiles = ['|', 'L', 'J'];
    let south = input.chars().nth(position + line_length).unwrap();
    if valid_south_tiles.contains(&south) {
        return (south, position + line_length);
    }

    unreachable!() //in theory
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day10_part1_test() {
        assert_eq!(run(), 8);
    }
}
