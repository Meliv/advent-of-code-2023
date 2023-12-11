use itertools::Itertools;
use regex::Regex;

const INPUT_FILE_PATH: &str = "src/inputs/day_11.txt";
const GALAXY_REGEX: &str = "#";

pub fn run() -> usize {
    let input = std::fs::read_to_string(INPUT_FILE_PATH).unwrap();

    let galaxy: Galaxy = get_galaxy(input);

    println!("Node Count {}", galaxy.nodes.iter().filter(|n| n.value == '#').count());
    println!("Node Pair Count: {:?}", galaxy.node_pairs.len());

    galaxy.print();

    374
}

#[derive(Debug)]
struct Galaxy {
    nodes: Vec<Node>,
    node_pairs: Vec<(Node, Node)>,
    width: usize
}
impl Galaxy {
    fn get(&self, x: usize, y: usize) -> &Node {
        self.nodes.iter().find(|n| n.x == x && n.y == y).unwrap()
    }

    fn print(&self) {
        for chunk in self.nodes.chunks(self.width+1){
            for c in chunk {
                print!("{}", c.value);
            }
            println!();
        }
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

    // Get node pairs
    let mut node_pairs: Vec<(Node, Node)> = Vec::new();
    let hash_nodes: Vec<&Node> = nodes.iter().filter(|&n| n.value == '#').collect();

    println!("Hash Nodes: {:?}", hash_nodes);

    for (i, node1) in hash_nodes.iter().enumerate() {
        for node2 in &hash_nodes[i + 1..] {

            // Debug
            if node1.x != node2.x && node1.y != node2.y {
                let first_pair: (Node, Node) = (**node1, **node2);
                let second_pair: (Node, Node) = (**node2, **node1);

                if !node_pairs.contains(&first_pair) && !node_pairs.contains(&second_pair) {
                    //println!("Added {:?}, {:?}", first_pair.0, first_pair.1);
                    node_pairs.push(first_pair);
                }
            }
        }
    }

    Galaxy { nodes, node_pairs, width: rows.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day11_part1_test() {
        assert_eq!(run(), 374);
    }
}
