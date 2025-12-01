use std::fs::read_to_string;

use crate::{aoc::Aoc, day_01::DayOne};

mod aoc;
mod day_01;

fn main() {
    let days = [DayOne::new];

    for (index, init) in days.into_iter().enumerate() {
        let day_num = index + 1;

        let input = read_to_string(format!("inputs/day_{:02}.txt", day_num)).unwrap();

        let mut day = init(input);

        println!("Day {} part one: {}", day_num, day.part_one());
        println!("Day {} part two: {}", day_num, day.part_two());
    }
}
