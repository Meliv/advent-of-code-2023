use itertools::Itertools;

const INPUT_FILE_PATH: &str = "src/inputs/day_11.txt";

pub fn run() -> usize {
    let input = std::fs::read_to_string(INPUT_FILE_PATH).unwrap();

    let result = get_universe(input, 1000000).get_sum_shortest_paths();

    println!("Result {}", result);
    result
}

#[derive(Debug)]
struct Universe {
    galaxies: Vec<Node>,
    galaxy_pairs: Vec<(Node, Node)>,
}
impl Universe {
    fn get_sum_shortest_paths(&self) -> usize {
        self.galaxy_pairs
            .iter()
            .map(|(a, b)| a.x.abs_diff(b.x) + a.y.abs_diff(b.y))
            .sum()
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Node {
    x: usize,
    y: usize,
    value: char,
}

fn get_universe(input: String, exp_factor: usize) -> Universe {
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

    let mut rows_to_expand: Vec<usize> = vec![];
    for (i, line) in input.lines().enumerate() {
        rows.push(line.chars().collect_vec());

        if !line.contains('#') {
            rows_to_expand.push(i)
        }
    }

    // Get nodes
    let mut galaxies: Vec<Node> = vec![];
    for (y, row) in rows.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if c != &'#' {
                continue;
            }

            let x_exp = cols_to_expand.iter().filter(|c| c < &&x).count() * (exp_factor - 1);
            let y_exp = rows_to_expand.iter().filter(|c| c < &&y).count() * (exp_factor - 1);

            galaxies.push(Node {
                x: x_exp + x,
                y: y_exp + y,
                value: *c,
            });
        }
    }

    let mut galaxy_pairs: Vec<(Node, Node)> = Vec::new();

    for (i, node1) in galaxies.iter().enumerate() {
        for node2 in &galaxies[i + 1..] {
            galaxy_pairs.push((*node1, *node2));
        }
    }

    Universe { galaxies, galaxy_pairs }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day11_part2_test() {
        assert_eq!(run(), 692506533832);
    }
}
