#![allow(unused_imports)]
#[allow(dead_code)]

mod modules {
    pub mod day_1_part_1;
    pub mod day_1_part_2;
    pub mod day_2_part_1;
    pub mod day_2_part_2;
    pub mod day_3_part_1;
    pub mod day_3_part_2;
    pub mod day_4_part_1;
    pub mod day_4_part_2;
    pub mod day_5_part_1;
    pub mod day_5_part_2;
    pub mod day_7_part_1;
    pub mod day_7_part_2;
}

use modules::day_1_part_1;
use modules::day_1_part_2;
use modules::day_2_part_1;
use modules::day_2_part_2;
use modules::day_3_part_1;
use modules::day_3_part_2;
use modules::day_4_part_1;
use modules::day_4_part_2;
use modules::day_5_part_1;
use modules::day_5_part_2;
use modules::day_7_part_2;

fn main() {
    day_7_part_2::run();
}