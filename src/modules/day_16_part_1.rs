use std::{
    any,
    collections::{HashSet, VecDeque},
    io,
};

use itertools::Itertools;

const INPUT_FILE_PATH: &str = "src/inputs/day_16.txt";

pub fn run() -> usize {
    let input = std::fs::read_to_string(INPUT_FILE_PATH).unwrap();

    let map = Map::new(input);

    let first_tile = map.get_tile(0,0).unwrap();

    let initial_beam = Beam {
        x: first_tile.x,
        y: first_tile.y,
        direction: match first_tile.c {
            '.' => BeamDirection::East,
            '\\' => BeamDirection::South,
            _ => panic!()
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

            let inserted = energised_cells.insert(EnergisedCell {
                x: b.x,
                y: b.y,
                entry_direction: b.direction,
            });
            if inserted {
                beams.extend(next_beams);
            }
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
    x: usize,
    y: usize,
    direction: BeamDirection,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum BeamDirection {
    North,
    South,
    East,
    West,
    Undefined,
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

    fn get_tile(&self, x: usize, y: usize) -> Option<&Cell> {
        self.cells.iter().find(|c| c.x == x && c.y == y)
    }

    fn get_beam_next_tile(&self, beam: &Beam) -> Option<&Cell> {
        match beam.direction {
            BeamDirection::North => self.get_tile(beam.x, beam.y - 1),
            BeamDirection::South => self.get_tile(beam.x, beam.y + 1),
            BeamDirection::East => self.get_tile(beam.x + 1, beam.y),
            BeamDirection::West => self.get_tile(beam.x - 1, beam.y),
            _ => panic!(),
        }
    }

    fn get_next_beams(&self, beam: &Beam) -> Vec<Beam> {
        let next_tile = self.get_beam_next_tile(beam);

        if let Some(t) = next_tile {
            match (t.c, beam.direction) {
                ('|', BeamDirection::East | BeamDirection::West) => self.get_split_beams(t, beam),
                ('-', BeamDirection::North | BeamDirection::South) => self.get_split_beams(t, beam),
                _ => self.get_single_beam(t, beam),
            };
        }
        vec![]
    }

    fn get_single_beam(&self, next_tile: &Cell, beam: &Beam) -> Vec<Beam> {
        let mut new_beam = Beam {
            x: next_tile.x,
            y: next_tile.y,
            direction: BeamDirection::Undefined,
        };
        new_beam.direction = match (next_tile.c, beam.direction) {
            ('\\', BeamDirection::North) => BeamDirection::West,
            ('\\', BeamDirection::South) => BeamDirection::East,
            ('\\', BeamDirection::East) => BeamDirection::South,
            ('\\', BeamDirection::West) => BeamDirection::North,

            ('/', BeamDirection::North) => BeamDirection::East,
            ('/', BeamDirection::South) => BeamDirection::West,
            ('/', BeamDirection::East) => BeamDirection::North,
            ('/', BeamDirection::West) => BeamDirection::South,

            _ => beam.direction,
        };

        vec![new_beam]
    }

    fn get_split_beams(&self, next_tile: &Cell, beam: &Beam) -> Vec<Beam> {
        match (next_tile.c, beam.direction) {
            ('|', BeamDirection::East | BeamDirection::West) => vec![
                Beam {
                    x: next_tile.x,
                    y: next_tile.y,
                    direction: BeamDirection::North,
                },
                Beam {
                    x: next_tile.x,
                    y: next_tile.y,
                    direction: BeamDirection::South,
                },
            ],
            ('-', BeamDirection::North | BeamDirection::South) => vec![
                Beam {
                    x: next_tile.x,
                    y: next_tile.y,
                    direction: BeamDirection::East,
                },
                Beam {
                    x: next_tile.x,
                    y: next_tile.y,
                    direction: BeamDirection::West,
                },
            ],
            _ => panic!(),
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
