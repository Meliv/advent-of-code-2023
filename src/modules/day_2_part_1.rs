static INPUT_FILE_PATH: &str = "src/inputs/day_2.txt";

pub fn run() {
    let input = std::fs::read_to_string(INPUT_FILE_PATH).unwrap();

    for x in input.lines().into_iter() {
        println!("{}", x);
    }
}
