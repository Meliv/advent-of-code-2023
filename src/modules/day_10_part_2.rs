use itertools::Itertools;
use num::complex::ComplexFloat;
use regex::Regex;

const INPUT_FILE_PATH: &str = "src/inputs/day_10.txt";
const START_REGEX: &str = "S";

pub fn run() -> usize {
    let mut input = std::fs::read_to_string(INPUT_FILE_PATH).unwrap();
    let line_length: isize = input.lines().next().unwrap().len() as isize;
    input = input.replace(['\r', '\n'], "");
    let exp = Regex::new(START_REGEX).unwrap();
    let start = exp.find(&input).unwrap();

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

    let mut replaced_input: Vec<char> = input.chars().collect();

    while current_tile != 'S' {
        // Debug
        //println!("Step {}: At {} with {}. Last pos {}", result, current_pos, current_tile, last_pos);

        next_pos = match current_tile {
            '|' => {
                // North/South
                match current_pos > last_pos {
                    true => south,
                    false => north,
                }
            }
            '-' => {
                // West/East
                match current_pos > last_pos {
                    true => east,
                    false => west,
                }
            }
            'L' => {
                // North/East
                match current_pos < last_pos {
                    true => north,
                    false => east,
                }
            }
            'J' => {
                // North/West
                match current_pos - 1 > last_pos {
                    true => west,
                    false => north,
                }
            }
            '7' => {
                // South/West
                match current_pos > last_pos {
                    true => south,
                    false => west,
                }
            }
            'F' => {
                //South/East
                match current_pos + 1 < last_pos {
                    true => east,
                    false => south,
                }
            }
            _ => panic!("Tile: {}, Pos {}", current_tile, current_pos),
        };

        last_pos = current_pos;
        current_pos += next_pos;
        current_tile = *replaced_input.get(current_pos as usize).unwrap();
        // Part 2
        if let Some(y) = replaced_input.get_mut(last_pos as usize) {
            *y = 'S'
        }
    }
    let mut result = 0;

    //println!("Line Length {}", line_length);

    for line in replaced_input.chunks(line_length as usize) {
        // Debug
        for &c in line {
            print!("{} ", &c);
        }

        let mut in_loop = line.starts_with(&['S']);
        for w in line.chunks(2) {
            if w.len() == 1 {
                break;
            }

            match w {
                ['S', 'S'] => {}
                [_, 'S'] => {
                    if in_loop {
                        result += 1;
                        in_loop = false
                    } else {
                        in_loop = true
                    }
                }
                ['S', _] => {
                    if in_loop {
                        in_loop = false
                    } else {
                        in_loop = true;
                        result += 1
                    }
                }
                [_, _] => {
                    if in_loop {
                        result += 2
                    }
                }
                _ => {}
            }
        }

        println!("Running Count: {}", result);
    }

    println!();

    println!("Result {}", result);

    result
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
    fn day10_part2_test() {
        assert_eq!(run(), 4);
    }
}
