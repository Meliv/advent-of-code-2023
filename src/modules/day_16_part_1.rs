use std::{
    collections::{HashSet, VecDeque},
    io,
};

use itertools::Itertools;

const INPUT_FILE_PATH: &str = "src/inputs/day_16.txt";

pub fn run() -> usize {
    let input = std::fs::read_to_string(INPUT_FILE_PATH)
        .unwrap()
        .replace(['\r', '\n'], "");

    let first_beam = match input.chars().next().unwrap() {
        '.' => Beam {
            current_tile_char: '.',
            current_tile_index: 0,
            direction: BeamDirection::East,
        },
        '\\' => Beam {
            current_tile_char: '\\',
            current_tile_index: 0,
            direction: BeamDirection::South,
        },
        _ => panic!(),
    };

    let mut beams: Vec<Beam> = vec![first_beam];

    let map = Map::new(input);

    let mut energised_cells: HashSet<isize> = HashSet::new();
    energised_cells.insert(0);

    while !beams.is_empty() {
        let beam = beams.get(0).unwrap();

        let next_beam = map.get_next_beams(beam);

        if let Some(bs) = next_beam {
            energised_cells.insert(bs.first().unwrap().current_tile_index);
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

#[derive(Copy, Clone, PartialEq)]
enum BeamDirection {
    // 10 chars long in test input
    North = -110,
    South = 110,
    East = 1,
    West = -1,
}

struct Map {
    tiles: String,
}

impl Map {
    fn new(input: String) -> Map {
        Map {
            tiles: input.clone(),
        }
    }

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

    fn get_next_beams(&self, beam: &Beam) -> Option<Vec<Beam>> {
        // Need to make sure the beam can't wrap to the previous line
        let line_start: isize = (beam.current_tile_index / 110) * 110;
        let line_end: isize = line_start + 109;

        let current_tile_index: isize = beam.current_tile_index + beam.direction as isize;

        if (beam.direction == BeamDirection::East || beam.direction == BeamDirection::West)
            && (current_tile_index < line_start || current_tile_index > line_end)
        {
            return None;
        }

        match current_tile_index < 0 || current_tile_index >= self.tiles.len() as isize {
            false => {
                let current_tile_char =
                    self.tiles.chars().nth(current_tile_index as usize).unwrap();
                match (current_tile_char, beam.direction) {
                    ('/', BeamDirection::East) => Some(vec![Beam {
                        current_tile_char,
                        current_tile_index,
                        direction: BeamDirection::North,
                    }]),
                    ('/', BeamDirection::South) => Some(vec![Beam {
                        current_tile_char,
                        current_tile_index,
                        direction: BeamDirection::West,
                    }]),
                    ('/', BeamDirection::West) => Some(vec![Beam {
                        current_tile_char,
                        current_tile_index,
                        direction: BeamDirection::South,
                    }]),
                    ('/', BeamDirection::North) => Some(vec![Beam {
                        current_tile_char,
                        current_tile_index,
                        direction: BeamDirection::East,
                    }]),

                    ('\\', BeamDirection::East) => Some(vec![Beam {
                        current_tile_char,
                        current_tile_index,
                        direction: BeamDirection::South,
                    }]),
                    ('\\', BeamDirection::South) => Some(vec![Beam {
                        current_tile_char,
                        current_tile_index,
                        direction: BeamDirection::East,
                    }]),
                    ('\\', BeamDirection::West) => Some(vec![Beam {
                        current_tile_char,
                        current_tile_index,
                        direction: BeamDirection::North,
                    }]),
                    ('\\', BeamDirection::North) => Some(vec![Beam {
                        current_tile_char,
                        current_tile_index,
                        direction: BeamDirection::West,
                    }]),

                    ('|', BeamDirection::West | BeamDirection::East) => Some(vec![
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
                    ]),

                    ('-', BeamDirection::North | BeamDirection::South) => Some(vec![
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
                    ]),
                    ('.', BeamDirection::East | BeamDirection::North | BeamDirection::South | BeamDirection::West) => Some(vec![Beam {
                        current_tile_char,
                        current_tile_index,
                        direction: beam.direction,
                    }]),
                    
                    ('|', BeamDirection::North | BeamDirection::South) => Some(vec![
                        Beam {
                            current_tile_char,
                            current_tile_index,
                            direction: beam.direction,
                        }
                    ]),

                    ('-', BeamDirection::West | BeamDirection::East) => Some(vec![
                        Beam {
                            current_tile_char,
                            current_tile_index,
                            direction: beam.direction,
                        }
                    ]),
                    _ => panic!()
                }
            }
            true => None,
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
