use std::collections::VecDeque;

use itertools::Itertools;
use num::complex::ComplexFloat;

const INPUT_FILE_PATH: &str = "src/inputs/day_10.txt";
const START_REGEX: &str = "S";

fn replace_loop_pipe(input: &mut Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut filtered_map = input.clone();
    let mut start: (isize, isize) = (0, 0);
    for (y, col) in filtered_map.iter().enumerate() {
        for (x, c) in col.iter().enumerate() {
            if c == &'S' {
                start = (x as isize, y as isize);
            }
        }
    }

    let start_direction = get_starting_direction(&filtered_map, start);

    // Mutable in the loop for pathfinding
    let mut current_tile: char = start_direction.0;
    let mut current_pos: (isize, isize) = (start_direction.1, start_direction.2);
    let mut last_pos: (isize, isize) = start;
    let mut next_pos: (isize, isize);

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
        current_tile = get_x_y(&filtered_map, current_pos.0, current_pos.1);
        // Part 2
        set_x_y(&mut filtered_map, last_pos.0, last_pos.1, 'S');
    }
    filtered_map
}

pub fn run() -> usize {
    let string_input = std::fs::read_to_string(INPUT_FILE_PATH).unwrap();
    let row_count: usize = string_input.lines().count();

    let mut map: Vec<Vec<char>> = vec![];

    for line in string_input.lines() {
        map.push(
            line.chars()
                .filter(|c| c != &'\r' && c != &'\n')
                .collect_vec(),
        );
    }

    let filtered_map = replace_loop_pipe(&mut map);

    for x in &filtered_map {
        for c in x {
            print!("{}", c);
        }
        println!();
    }
    println!();

    let result = ray_count(&map, &filtered_map);

    for x in &map {
        for c in x {
            print!("{}", c);
        }
        println!();
    }

    println!("Result: {}", result);

    result
}

fn ray_count(map: &Vec<Vec<char>>, filtered_map: &Vec<Vec<char>>) -> usize {
    let mut match_count = 0;
    for (i_y, row) in filtered_map.iter().enumerate() {
        for (i_x, c) in row.iter().enumerate() {

            if c == &'S'{
                continue;
            }
            //           1111111111
            // 01234567890123456789
            // L--J.L7.X.LJS7F-7L7.
            let mut x_count = i_x as isize + 1;
            let mut north_count = 0;
            let mut south_count = 0;
            while x_count < row.len() as isize {
                let char = get_x_y(map, x_count, i_y as isize);
                if get_x_y(filtered_map, x_count, i_y as isize) != 'S' {
                    x_count += 1;
                    continue;

                }
                match char {
                    '|' => north_count += 1,
                    'L' => north_count += 1,
                    'J' => north_count += 1,
                    _ => {}
                }

                match char {
                    '7' => south_count += 1,
                    '|' => south_count += 1,
                    'F' => south_count += 1,
                    'S' => south_count += 1,
                    _ => {}
                }

                //

                x_count += 1;
            }
            let odd_right = north_count > 0 && south_count > 0 && north_count % 2 == 1 && south_count % 2 == 1;

            // 0123456789
            // OOOOOXOOOO
            x_count = i_x as isize - 1;
            north_count = 0;
            south_count = 0;

            while x_count >= 0 {
                let char = get_x_y(map, x_count, i_y as isize);
                if get_x_y(filtered_map, x_count, i_y as isize) != 'S' {
                    x_count -= 1;
                    continue;
                }
                match char {
                    '|' => north_count += 1,
                    'L' => north_count += 1,
                    'J' => north_count += 1,
                    _ => {}
                }

                match char {
                    '7' => south_count += 1,
                    '|' => south_count += 1,
                    'F' => south_count += 1,
                    'S' => south_count += 1,
                    _ => {}
                }

                x_count -= 1;
            }
            let odd_left = north_count > 0 && south_count > 0 && north_count % 2 == 1 && south_count % 2 == 1;

            if odd_left && odd_right {
                println!("match at x {} y {}", i_x, i_y);
                match_count += 1;
            }
        }
    }

    match_count
}

fn get_x_y(input: &Vec<Vec<char>>, x: isize, y: isize) -> char {
    let rows = input.get(y as usize).unwrap();
    *rows.get(x as usize).unwrap()
}

fn set_x_y(input: &mut Vec<Vec<char>>, x: isize, y: isize, v: char) {
    println!("set_x_y x:{} y:{}", x, y);
    let rows = input.get_mut(y as usize).unwrap();
    *rows.get_mut(x as usize).unwrap() = v;
}

fn get_starting_direction(
    input: &Vec<Vec<char>>,
    position: (isize, isize),
) -> (char, isize, isize) {
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
        assert_eq!(run(), 541);
    }
}
