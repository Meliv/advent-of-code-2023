static INPUT_FILE_PATH: &str = "src/inputs/day_9.txt";

pub fn run() -> isize {
    let input = std::fs::read_to_string(INPUT_FILE_PATH).unwrap();

    let sequences: Vec<Vec<isize>> = input
        .lines()
        .map(|line| line.split(' ').map(|c| c.parse().unwrap()).collect())
        .collect();

    let result: isize = sequences
        .iter()
        .map(|s| s.last().unwrap() + get_next_sequence_no(s))
        .sum();

    println!("Result {}", result);

    result
}

fn get_next_sequence_no(sequence: &Vec<isize>) -> isize {
    let mut steps: Vec<isize> = vec![];

    let mut count = 0;
    while count < sequence.len() - 1 {
        let a = *sequence.get(count).unwrap();
        let b = *sequence.get(count + 1).unwrap();
        steps.push(b - a);

        count += 1;
    }

    if steps.iter().all(|i| i == &0) {
        return 0;
    }

    steps.last().unwrap() + get_next_sequence_no(&steps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day9_part1_test() {
        assert_eq!(run(), 1938800261);
    }
}
