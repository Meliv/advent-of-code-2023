use core::num;

use regex::Regex;

static INPUT_FILE_PATH: &str = "src/inputs/day_6.txt";
const REGEX_STRING: &str = r"(\b\d+\b)";

pub fn run() -> usize {
    let input = std::fs::read_to_string(INPUT_FILE_PATH).unwrap();
    let result = get_races(input).iter().fold(1, |acc, r| acc * solve(r));
    println!("Result: {}", result);
    result
}

fn solve(race: &Race) -> usize {
    let t = race.time;
    let d = race.distance;

    let u = (t + (t * t - 4. * 1. * d).sqrt()) / 2.;
    let l = (t - (t * t - 4. * 1. * d).sqrt()) / 2.;

    let mut r = 0.;
    if u.fract() == 0. {
        r = 2.;
    }

    (u.floor() - l.ceil() - r + 1.) as usize
}

fn get_races(input: String) -> Vec<Race> {
    let mut races: Vec<Race> = vec![];
    let mut times: Vec<usize> = vec![];
    let mut distances: Vec<usize> = vec![];

    let exp = Regex::new(REGEX_STRING).unwrap();

    // Too lazy to do something properly
    let t_line = input.lines().next().unwrap();
    for c in exp.captures_iter(t_line) {
        times.push(c.get(0).unwrap().as_str().parse().unwrap());
    }

    let d_line = input.lines().nth(1).unwrap(); // Why can't I just do .next() here?
    for c in exp.captures_iter(d_line) {
        distances.push(c.get(0).unwrap().as_str().parse().unwrap());
    }

    for (i, t) in times.iter().enumerate() {
        races.push(Race {
            time: *t as f64,
            distance: *distances.get(i).unwrap() as f64,
        })
    }

    races
}

#[derive(Debug)]
struct Race {
    time: f64,
    distance: f64,
}

mod tests {
    use super::*;

    #[test]
    fn day6_part1_test() {
        assert_eq!(run(), 0);
    }
}
