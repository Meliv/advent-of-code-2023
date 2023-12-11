use itertools::Itertools;
use regex::Regex;
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

const INPUT_FILE_PATH: &str = "src/inputs/day_11.txt";
const GALAXY_REGEX: &str = "#";

pub fn run() -> usize {
    let input = std::fs::read_to_string(INPUT_FILE_PATH).unwrap();

    println!("Start Parse");
    let galaxy = get_galaxy(input);
    println!("End Parse");

    println!("Start Pathfind");
    let result = galaxy.get_sum_shortest_paths();
    println!("End Pathfind");

    println!("Result {}", result);
    result
}

#[derive(Debug)]
struct Galaxy {
    nodes: Vec<Node>,
    node_pairs: Vec<(Node, Node)>,
    width: usize,
    height: usize
}
impl Galaxy {
    fn get(&self, x: usize, y: usize) -> &Node {
        self.nodes.iter().find(|n| n.x == x && n.y == y).unwrap()
    }

    fn print(&self) {
        for chunk in self.nodes.chunks(self.width) {
            for c in chunk {
                print!("{}", c.value);
            }
            println!();
        }
    }

    fn get_sum_shortest_paths(&self) -> usize {
        let mut result = 0;
        let astar = AStar::new(self);
        for node_pair in &self.node_pairs {
            result += astar.find_path(node_pair.0, node_pair.1).unwrap();
        }
        result
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash)]
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

    for (i, node1) in hash_nodes.iter().enumerate() {
        for node2 in &hash_nodes[i + 1..] {
            let first_pair: (Node, Node) = (**node1, **node2);
            let second_pair: (Node, Node) = (**node2, **node1);

            if !node_pairs.contains(&first_pair) && !node_pairs.contains(&second_pair) {
                node_pairs.push(first_pair);
            }
        }
    }

    Galaxy {
        nodes,
        node_pairs,
        width: rows.first().unwrap().len(),
        height: rows.len()
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct Path {
    node: Node,
    cost: usize,
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct AStar<'a> {
    galaxy: &'a Galaxy,
    width: usize,
    height: usize,
}

impl<'a> AStar<'a> {
    fn new(galaxy: &'a Galaxy) -> AStar<'a> {
        let height = galaxy.height;
        let width = galaxy.width;
        AStar { galaxy, width, height }
    }

    fn heuristic(&self, from: Node, to: Node) -> usize {
        let dx = if from.x > to.x { from.x - to.x } else { to.x - from.x };
        let dy = if from.y > to.y { from.y - to.y } else { to.y - from.y };
        dx + dy
    }

    fn find_path(&self, start: Node, goal: Node) -> Option<usize> {
        let mut open_set = BinaryHeap::new();
        let mut came_from: HashMap<Node, Node> = HashMap::new();
        let mut g_score: HashMap<Node, usize> = HashMap::new();

        open_set.push(Path { node: start, cost: 0 });
        g_score.insert(start, 0);

        while let Some(Path { node, cost }) = open_set.pop() {
            if node == goal {
                let mut path = vec![];
                let mut current = node;
                while let Some(&prev) = came_from.get(&current) {
                    path.push(prev);
                    current = prev;
                }
                path.reverse();
                return Some(path.len());
            }

            for neighbor in self.get_neighbors(node) {
                let tentative_g_score = g_score[&node] + 1;
                if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&usize::MAX) {
                    came_from.insert(neighbor, node);
                    g_score.insert(neighbor, tentative_g_score);
                    let priority = tentative_g_score + self.heuristic(neighbor, goal);
                    open_set.push(Path { node: neighbor, cost: priority });
                }
            }
        }

        None
    }

    fn get_neighbors(&self, node: Node) -> Vec<Node> {
        let mut neighbors = Vec::new();
        if node.x > 0 {
            neighbors.push(Node { x: node.x - 1, y: node.y, value: node.value });
        }
        if node.x < self.width - 1 {
            neighbors.push(Node { x: node.x + 1, y: node.y, value: node.value });
        }
        if node.y > 0 {
            neighbors.push(Node { x: node.x, y: node.y - 1, value: node.value });
        }
        if node.y < self.height - 1 {
            neighbors.push(Node { x: node.x, y: node.y + 1, value: node.value });
        }
        neighbors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day11_part1_test() {
        assert_eq!(run(), 374);
    }
}
