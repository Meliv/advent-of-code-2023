use std::collections::VecDeque;

use itertools::Itertools;
use num::complex::ComplexFloat;

const INPUT_FILE_PATH: &str = "src/inputs/day_10.txt";
const START_REGEX: &str = "S";

fn replace_loop_pipe(input: &mut Vec<Vec<char>>) {
    let mut start = (0,0);
    for (y, col) in input.iter().enumerate() {
        for (x, c) in col.iter().enumerate() {
            if c == &'S' {
                start = (x, y);
            }
        }
    }

    let start_direction = get_starting_direction(&input, start);

    // Mutable in the loop for pathfinding
    let mut current_tile: char = start_direction.0;
    let mut current_pos: (usize, usize) = (start_direction.1, start_direction.2);
    let mut last_pos: (usize, usize) = start;
    let mut next_pos: (usize, usize);

    while current_tile != 'S' {
        next_pos = match current_tile {
            '|' => {
                // North/South
                match current_pos.1 > last_pos.1 {
                    true => (current_pos.0, current_pos.1 + 1),
                    false => (current_pos.0, current_pos.1 - 1),
                }
            }
            '-' => {
                // West/East
                match current_pos.0 > last_pos.0 {
                    true => (current_pos.0 + 1, current_pos.1),
                    false => (current_pos.0 - 1, current_pos.1),
                }
            }
            'L' => {
                // North/East
                match current_pos.0 < last_pos.0 {
                    true => (current_pos.0, current_pos.1 - 1),
                    false => (current_pos.0 + 1, current_pos.1),
                }
            }
            'J' => {
                // North/West
                match current_pos.1 > last_pos.1 {
                    true => (current_pos.0 - 1, current_pos.1),
                    false => (current_pos.0, current_pos.1 - 1),
                }
            }
            '7' => {
                // South/West
                match current_pos.0 > last_pos.0 {
                    true => (current_pos.0, current_pos.1 + 1),
                    false => (current_pos.0 - 1, current_pos.1),
                }
            }
            'F' => {
                //South/East
                match current_pos.1 < last_pos.1 {
                    true => (current_pos.0 + 1, current_pos.1),
                    false => (current_pos.0, current_pos.1 + 1),
                }
            }
            _ => panic!("Tile: {}, Pos {:?}", current_tile, current_pos),
        };

        last_pos = current_pos;
        current_pos = next_pos;
        current_tile = get_x_y(&input, current_pos.0, current_pos.1);
        // Part 2
        set_x_y(input, last_pos.0, last_pos.1, 'S');
    }
}

pub fn run() -> usize {
    let string_input = std::fs::read_to_string(INPUT_FILE_PATH).unwrap();
    let row_count: usize = string_input.lines().count();
    
    let mut map: Vec<Vec<char>> = vec![vec![]];
    
    for line in string_input.lines() {
        map.push(
            line.chars()
            .filter(|c| c != &'\r' && c != &'\n')
            .collect_vec(),
        );
    }
    let col_count: usize = string_input.lines().next().unwrap().len();

    
    replace_loop_pipe(&mut map);

    let x = (0,0);
    flood_fill(&mut map, col_count, row_count, x.0, x.1);

    for x in map {
        for c in x {
            print!("{}", c);
        }
        println!();
    }

    let result: usize = 0;
    result
}

fn flood_fill(input: &mut Vec<Vec<char>>, rowCount: usize, colCount: usize, x: usize, y: usize) {
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    queue.push_back((x, y));
}

fn get_x_y(input: &Vec<Vec<char>>, x: usize, y: usize) -> char {
    let rows = input.get(y).unwrap();
    *rows.get(x).unwrap()
}

fn set_x_y(input: &mut Vec<Vec<char>>, x: usize, y: usize, v: char) {
    let rows = input.get_mut(y).unwrap();
    *rows.get_mut(x).unwrap() = v;
}

fn get_starting_direction(input: &Vec<Vec<char>>, position: (usize, usize)) -> (char, usize, usize) {
    let valid_west_tiles = ['-', 'L', 'F'];
    let west = get_x_y(&input, position.0 - 1, position.1);
    if valid_west_tiles.contains(&west) {
        return (west, position.0 - 1, position.1);
    }

    let valid_east_tiles = ['-', 'J', '7'];
    let east = get_x_y(input, position.0 + 1, position.1);
    if valid_east_tiles.contains(&east) {
        return (east, position.0 + 1, position.1);
    }

    let valid_north_tiles = ['|', 'J', 'F'];
    let north = get_x_y(input, position.0, position.1 - 1);
    if valid_north_tiles.contains(&north) {
        return (north, position.0, position.1 - 1);
    }

    let valid_south_tiles = ['|', 'L', 'J'];
    let south = get_x_y(input, position.0, position.1 + 1);
    if valid_south_tiles.contains(&south) {
        return (south, position.0, position.1 + 1);
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day10_part2_test() {
        assert_eq!(run(), 4);
    }
}
