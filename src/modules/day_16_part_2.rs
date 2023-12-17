use std::collections::HashSet;

const INPUT_FILE_PATH: &str = "src/inputs/day_16.txt";

pub fn run() -> usize {
    let input = std::fs::read_to_string(INPUT_FILE_PATH).unwrap();

    let map = Map::new(input);

    let mut result = 0;

    let mut try_beams: Vec<Beam> = vec![];

    let range: Vec<isize> = (0..10).collect();
    try_beams.extend(range.iter().map(|i| Beam {
        x: *i,
        y: 0,
        direction: BeamDirection::South,
    }));
    try_beams.extend(range.iter().map(|i| Beam {
        x: *i,
        y: 9,
        direction: BeamDirection::North,
    }));
    try_beams.extend(range.iter().map(|i| Beam {
        x: 0,
        y: *i,
        direction: BeamDirection::East,
    }));
    try_beams.extend(range.iter().map(|i| Beam {
        x: 9,
        y: *i,
        direction: BeamDirection::West,
    }));

    for initial_beam in try_beams {
        let mut beams: Vec<Beam> = vec![initial_beam];

        let mut energised_cells: HashSet<EnergisedCell> = HashSet::new();
        energised_cells.insert(EnergisedCell {
            x: initial_beam.x,
            y: initial_beam.y,
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
        }

        let try_beam_result = map.get_result(&energised_cells);

        if try_beam_result > result {
            result = try_beam_result;
        }

    }

    println!("Result: {}", result);
    result
}

impl Map {
    fn new(input: String) -> Map {
        let mut cells: Vec<Cell> = vec![];
        for (i_y, line) in input.lines().enumerate() {
            for (i_x, c) in line.chars().enumerate() {
                cells.push(Cell {
                    x: i_x as isize,
                    y: i_y as isize,
                    c,
                })
            }
        }

        Map { cells }
    }

    fn print(&self, energised_cells: &HashSet<EnergisedCell>) {
        let line_length = 10;
        for (i, c) in self.cells.iter().enumerate() {
            if i != 0 && i % line_length == 0 {
                println!();
            }

            if energised_cells.iter().any(|ec| ec.x == c.x && ec.y == c.y) {
                print!("X")
            } else {
                print!(".")
            }
        }

        println!();
    }

    fn get_result(&self, energised_cells: &HashSet<EnergisedCell>) -> usize {
        let mut result = 0;
        for c in self.cells.iter() {
            if energised_cells.iter().any(|ec| ec.x == c.x && ec.y == c.y) {
                result += 1;
            }
        }

        result
    }

    fn get_tile(&self, x: isize, y: isize) -> Option<&Cell> {
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
                ('|', BeamDirection::East | BeamDirection::West) => {
                    return self.get_split_beams(t, beam)
                }
                ('-', BeamDirection::North | BeamDirection::South) => {
                    return self.get_split_beams(t, beam)
                }
                _ => return self.get_single_beam(t, beam),
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

#[derive(Copy, Clone)]
struct Beam {
    x: isize,
    y: isize,
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
    x: isize,
    y: isize,
    c: char,
}

#[derive(Eq, PartialEq, Hash)]
struct EnergisedCell {
    x: isize,
    y: isize,
    entry_direction: BeamDirection,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day16_part2_test() {
        assert_eq!(run(), 7932);
    }
}
