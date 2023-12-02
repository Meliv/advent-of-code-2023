#![allow(unused_imports)]
#[allow(dead_code)]

mod modules {
    pub mod day_1_part_1;
    pub mod day_1_part_2;
    pub mod day_2_part_1;
}

use modules::day_1_part_1;
use modules::day_1_part_2;
use modules::day_2_part_1;

fn main() {
    //println!("{}", day_1_part_1::run());
    //println!("{}", day_1_part_2::run());
    day_2_part_1::run();
}