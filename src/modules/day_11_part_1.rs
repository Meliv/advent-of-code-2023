use itertools::Itertools;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

const INPUT_FILE_PATH: &str = "src/inputs/day_11.txt";
const GALAXY_REGEX: &str = "#";

pub fn run() -> usize {
    let input = std::fs::read_to_string(INPUT_FILE_PATH).unwrap();

    let result = get_galaxy(input).get_sum_shortest_paths();

    println!("Result {}", result);
    result
}

#[derive(Debug)]
struct Galaxy {
    nodes: Vec<Node>,
    node_pairs: Vec<(Node, Node)>,
}
impl Galaxy {
    fn get(&self, x: usize, y: usize) -> &Node {
        self.nodes.iter().find(|n| n.x == x && n.y == y).unwrap()
    }

    fn get_sum_shortest_paths(&self) -> usize {
        let mut result = 0;
        for pair in &self.node_pairs {

            /*
            println!();
            println!("{:?}", pair);
            
            println!("({}-{})+({}-{})",
            pair.0.x.max(pair.1.x),
            pair.0.x.min(pair.1.x),
            pair.0.y.max(pair.1.y),
            pair.0.y.min(pair.1.y));
            
            println!("{}+{}={}",
            pair.0.x.max(pair.1.x) - pair.0.x.min(pair.1.x),
            pair.0.y.max(pair.1.y) - pair.0.y.min(pair.1.y),
            (pair.0.x.max(pair.1.x) - pair.0.x.min(pair.1.x)) + (pair.0.y.max(pair.1.y) - pair.0.y.min(pair.1.y)));
            
            */
            result += (pair.0.x.max(pair.1.x) - pair.0.x.min(pair.1.x))
            + (pair.0.y.max(pair.1.y) - pair.0.y.min(pair.1.y)) - 1;
        }

        result - self.nodes.iter().filter(|n| n.value == '#').count() - 1
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
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

    let mut rows_to_expand: Vec<usize> = vec![];
    for (i, line) in input.lines().enumerate() {
        rows.push(line.chars().collect_vec());

        if !line.contains('#') {
            rows_to_expand.push(i)
        }
    }

    // Get nodes
    let mut nodes: Vec<Node> = vec![];

    println!("Cols to expand {:?}", cols_to_expand);
    println!("Rows to expand {:?}", rows_to_expand);

    for (y, row) in rows.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {

            let e_mod = 2; // Expect 1030
            let mut x_mod = cols_to_expand.iter().filter(|c| c < &&x).count() * e_mod;
            let mut y_mod = rows_to_expand.iter().filter(|c| c < &&y).count() * e_mod;

            x_mod = x_mod.saturating_sub(1);
            y_mod = y_mod.saturating_sub(1);


            nodes.push(Node { x: x_mod + x, y: y_mod + y, value: *c });
        }
    }

    // Get node pairs
    let mut node_pairs: Vec<(Node, Node)> = Vec::new();
    let hash_nodes: Vec<&Node> = nodes.iter().filter(|&n| n.value == '#').collect();

    for (i, node1) in hash_nodes.iter().enumerate() {
        for node2 in &hash_nodes[i + 1..] {
            let first_pair: (Node, Node) = (**node1, **node2);
            let second_pair: (Node, Node) = (**node2, **node1);

            if !node_pairs.contains(&first_pair) && !node_pairs.contains(&second_pair) {
                node_pairs.push(first_pair);
            }
        }
    }

    Galaxy { nodes, node_pairs }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day11_part1_test() {
        assert_eq!(run(), 9918828);
    }
}
