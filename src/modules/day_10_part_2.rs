use std::collections::VecDeque;

use itertools::Itertools;
use num::complex::ComplexFloat;

const INPUT_FILE_PATH: &str = "src/inputs/day_10.txt";
const START_REGEX: &str = "S";

fn replace_loop_pipe(input: &mut Vec<Vec<char>>) {
    let mut start: (isize, isize) = (0, 0);
    for (y, col) in input.iter().enumerate() {
        for (x, c) in col.iter().enumerate() {
            if c == &'S' {
                start = (x as isize, y as isize);
            }
        }
    }

    let start_direction = get_starting_direction(&input, start);

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
        current_tile = get_x_y(&input, current_pos.0, current_pos.1);
        // Part 2
        set_x_y(input, last_pos.0, last_pos.1, 'S');
    }
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
    let col_count: usize = string_input.lines().next().unwrap().len();

    replace_loop_pipe(&mut map);

    let mut yyy: Option<(isize, isize)> = None;
    for (i_x, row) in map.iter().enumerate() {
        let mut in_loop: bool = row.first().unwrap() == &'S';

        for (i_y, chunk) in row.chunks(2).enumerate() {
            match chunk {
                ['S', 'S'] => {}
                [_, 'S'] => in_loop = true,
                ['S', _] => {
                    if in_loop {
                        in_loop = false
                    } else {
                        yyy = Some((i_x as isize, i_y as isize + 1))
                    }
                }
                [_, _] => {
                    if in_loop {
                        yyy = Some((i_x as isize, i_y as isize + 1))
                    }
                }
                _ => unreachable!(),
            }
        }

        if yyy.is_some() {
            break;
        }
    }

    if yyy.is_none() {
        panic!("Couldn't find inner boundary");
    }
    
        for x in &map {
            for c in x {
                print!("{}", c);
            }
            println!();
        }
    //let test_x_y = (1, 1); // For testing
    flood_fill(&mut map, col_count, row_count, yyy.unwrap().0, yyy.unwrap().1);

    0
}

fn flood_fill(input: &mut Vec<Vec<char>>, row_count: usize, col_count: usize, x: isize, y: isize) {
    let mut queue: VecDeque<(isize, isize)> = VecDeque::new();
    let mut p: (isize, isize) = (x, y);
    queue.push_back(p);

    set_x_y(input, x, y, 'X');

    while queue.len() > 0 {
        let current = queue.pop_front().unwrap();

        // East
        if is_valid(input, row_count, col_count, current.0 + 1, current.1) {
            set_x_y(input, current.0 + 1, current.1, 'X');
            p = (current.0 + 1, current.1);
            queue.push_back(p);
        }

        // West
        if is_valid(input, row_count, col_count, current.0 - 1, current.1) {
            set_x_y(input, current.0 - 1, current.1, 'X');
            p = (current.0 - 1, current.1);
            queue.push_back(p);
        }

        // North
        if is_valid(input, row_count, col_count, current.0, current.1 - 1) {
            set_x_y(input, current.0, current.1 - 1, 'X');
            p = (current.0, current.1 - 1);
            queue.push_back(p);
        }

        // South
        if is_valid(input, row_count, col_count, current.0, current.1 + 1) {
            set_x_y(input, current.0, current.1 + 1, 'X');
            p = (current.0, current.1 + 1);
            queue.push_back(p);
        }
    }
}

fn is_valid(
    input: &mut Vec<Vec<char>>,
    row_count: usize,
    col_count: usize,
    x: isize,
    y: isize,
) -> bool {
    if x < 0
        || x as usize >= row_count
        || y < 0
        || y as usize >= col_count
        || get_x_y(input, x, y) == 'S'
        || get_x_y(input, x, y) == 'X'
    {
        return false;
    }

    true
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
        assert_eq!(run(), 4);
    }
}
