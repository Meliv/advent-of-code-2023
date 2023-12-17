use std::{
    collections::{HashSet, VecDeque},
    io,
};

use itertools::Itertools;

const INPUT_FILE_PATH: &str = "src/inputs/day_16.txt";

pub fn run() -> usize {
    let input = std::fs::read_to_string(INPUT_FILE_PATH).unwrap();

    let map = Map::new(input);

    let initial_beam = Beam {
        current_tile_char: input.chars().next().unwrap(),
        current_tile_index: 0,
        direction: match input.chars().next().unwrap() {
            '.' => BeamDirection::East,
            '\\' => BeamDirection::South,
        },
    };

    let mut beams: Vec<Beam> = vec![initial_beam];

    let mut energised_cells: HashSet<EnergisedCell> = HashSet::new();
    energised_cells.insert(EnergisedCell {
        x: 0,
        y: 0,
        entry_direction: initial_beam.direction,
    });

    while !beams.is_empty() {
        let beam = beams.first().unwrap();

        let next_beams = map.get_next_beams(beam);

        if !next_beams.is_empty() {

            let b = next_beams.first().unwrap();

            /*
            let ec = EnergisedCell {
            }
            */

            energised_cells.insert(next_beams.first().unwrap().current_tile_index);
            beams.extend(bs);
        }

        beams.remove(0);

        //map.print(&energised_cells);

        //let mut x = String::new();
        //_ = io::stdin().read_line(&mut x);
        if energised_cells.len() == 7496 {
            break;
        }
    }

    let result = energised_cells.len();

    println!("Result: {}", result);
    result
}

#[derive(Copy, Clone)]
struct Beam {
    current_tile_char: char,
    current_tile_index: isize,
    direction: BeamDirection,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum BeamDirection {
    // 10 chars long in test input
    North = -110,
    South = 110,
    East = 1,
    West = -1,
}

struct Map {
    cells: Vec<Cell>,
}
struct Cell {
    x: usize,
    y: usize,
    c: char,
}

#[derive(Eq, PartialEq, Hash)]
struct EnergisedCell {
    x: usize,
    y: usize,
    entry_direction: BeamDirection,
}

impl Map {
    fn new(input: String) -> Map {
        let mut cells: Vec<Cell> = vec![];
        for (i_y, line) in input.lines().enumerate() {
            for (i_x, c) in line.chars().enumerate() {
                cells.push(Cell { x: i_x, y: i_y, c })
            }
        }

        Map { cells }
    }

    /*
    fn print(&self, tiles: &HashSet<isize>) {
        let mut x: Vec<char> = self.tiles.chars().collect();
        for (i, c) in x.iter().enumerate() {
            if i != 0 && i % 110 == 0 {
                println!();
            }
            let ii = i as isize;
            if tiles.contains(&ii) {
                print!("X");
            } else {
                print!("{}", c);
            }
        }
        println!();
    }
    */

    fn get_tile(&self, x: usize, y: usize) -> Cell {
        *self.cells.iter().find(|c| c.x == x && c.y == y).unwrap()
    }

    fn get_next_beams(&self, beam: &Beam) -> Vec<Beam> {
        // Need to make sure the beam can't wrap to the previous line
        let line_start: isize = (beam.current_tile_index / 110) * 110;
        let line_end: isize = line_start + 109;

        let current_tile_index: isize = beam.current_tile_index + beam.direction as isize;

        if (beam.direction == BeamDirection::East || beam.direction == BeamDirection::West)
            && (current_tile_index < line_start || current_tile_index > line_end)
        {
            return vec![];
        }

        match current_tile_index < 0 || current_tile_index >= self.tiles.len() as isize {
            false => {
                let current_tile_char =
                    self.tiles.chars().nth(current_tile_index as usize).unwrap();
                match (current_tile_char, beam.direction) {
                    ('/', BeamDirection::East) => vec![Beam {
                        current_tile_char,
                        current_tile_index,
                        direction: BeamDirection::North,
                    }],
                    ('/', BeamDirection::South) => vec![Beam {
                        current_tile_char,
                        current_tile_index,
                        direction: BeamDirection::West,
                    }],
                    ('/', BeamDirection::West) => vec![Beam {
                        current_tile_char,
                        current_tile_index,
                        direction: BeamDirection::South,
                    }],
                    ('/', BeamDirection::North) => vec![Beam {
                        current_tile_char,
                        current_tile_index,
                        direction: BeamDirection::East,
                    }],

                    ('\\', BeamDirection::East) => vec![Beam {
                        current_tile_char,
                        current_tile_index,
                        direction: BeamDirection::South,
                    }],
                    ('\\', BeamDirection::South) => vec![Beam {
                        current_tile_char,
                        current_tile_index,
                        direction: BeamDirection::East,
                    }],
                    ('\\', BeamDirection::West) => vec![Beam {
                        current_tile_char,
                        current_tile_index,
                        direction: BeamDirection::North,
                    }],
                    ('\\', BeamDirection::North) => vec![Beam {
                        current_tile_char,
                        current_tile_index,
                        direction: BeamDirection::West,
                    }],

                    ('|', BeamDirection::West | BeamDirection::East) => vec![
                        Beam {
                            current_tile_char,
                            current_tile_index,
                            direction: BeamDirection::North,
                        },
                        Beam {
                            current_tile_char,
                            current_tile_index,
                            direction: BeamDirection::South,
                        },
                    ],

                    ('-', BeamDirection::North | BeamDirection::South) => vec![
                        Beam {
                            current_tile_char,
                            current_tile_index,
                            direction: BeamDirection::East,
                        },
                        Beam {
                            current_tile_char,
                            current_tile_index,
                            direction: BeamDirection::West,
                        },
                    ],

                    _ => vec![Beam {
                        current_tile_char,
                        current_tile_index,
                        direction: beam.direction,
                    }],
                    _ => panic!(),
                }
            }
            true => vec![],
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day16_part1_test() {
        assert_eq!(run(), 46);
    }
}
