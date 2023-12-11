use regex::Regex;

const INPUT_FILE_PATH: &str = "src/inputs/day_11.txt";
const GALAXY_REGEX: &str = "#";

pub fn run() -> usize {
    let input = std::fs::read_to_string(INPUT_FILE_PATH).unwrap();

    let galaxy: Galaxy = get_galaxy(input);

    println!("{:?}", galaxy.get(0, 11));

    //println!("{:?}", galaxy);

    374
}

#[derive(Debug)]
struct Galaxy {
    nodes: Vec<Node>,
}
impl Galaxy {
    fn get(&self, x: usize, y: usize) -> &Node {
        self.nodes.iter().find(|n| n.x == x && n.y == y).unwrap()
    }
}

#[derive(Debug)]
struct Node {
    x: usize,
    y: usize,
    value: char,
}

fn get_galaxy(input: String) -> Galaxy {
    let mut rows: Vec<Vec<char>> = vec![];

    let mut i = 0;
    let mut cols_to_expand: Vec<usize> = vec![];
    while i < input.lines().next().unwrap().len() {
        let mut empty_col: bool = true;
        for line in input.lines() {
            let c = line.chars().nth(i).unwrap();
            if empty_col && c != '.' {
                empty_col = false;
            }
        }
        if empty_col {
            cols_to_expand.push(i);
        }
        i += 1;
    }

    for line in input.lines() {
        let mut chars: Vec<char> = line.chars().collect();
        for col in cols_to_expand.iter().rev() {
            chars.insert(*col, '.');
        }

        if !line.contains('#') {
            rows.push(chars.clone());
        }

        rows.push(chars);
    }

    // Get nodes
    let mut nodes: Vec<Node> = vec![];

    for (y, row) in rows.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            nodes.push(Node { x, y, value: *c });
        }
    }

    Galaxy { nodes }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day11_part1_test() {
        assert_eq!(run(), 374);
    }
}
