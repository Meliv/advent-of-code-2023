use regex::Regex;

static INPUT_FILE_PATH: &str = "src/inputs/day_6.txt";
const REGEX_STRING: &str = r"(\b\d+\b)";

pub fn run() -> usize {
    let input = std::fs::read_to_string(INPUT_FILE_PATH).unwrap();
    let result = solve(&get_race(input));
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

fn get_race(input: String) -> Race {
    let exp = Regex::new(REGEX_STRING).unwrap();

    // Too lazy to do something properly
    let time: String = exp
        .find_iter(input.lines().next().unwrap())
        .map(|m| m.as_str())
        .collect();

    let distance: String = exp
        .find_iter(input.lines().nth(1).unwrap())
        .map(|m| m.as_str())
        .collect();

    Race {
        time: time.parse().unwrap(),
        distance: distance.parse().unwrap(),
    }
}

#[derive(Debug)]
struct Race {
    time: f64,
    distance: f64,
}

mod tests {
    use super::*;

    #[test]
    fn day6_part2_test() {
        assert_eq!(run(), 21039729);
    }
}
