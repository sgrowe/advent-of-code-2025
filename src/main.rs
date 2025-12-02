use std::fs::read_to_string;

use crate::{aoc::Aoc, day_01::DayOne, day_02::DayTwo};

mod aoc;
mod day_01;
mod day_02;

fn main() {
    solve::<DayOne>(1);
    solve::<DayTwo>(2);
}

fn solve<Day: Aoc>(day: usize) {
    let input = read_to_string(format!("inputs/day_{:02}.txt", day)).unwrap();

    let mut solver = Day::new(input);

    println!("Day {} part one: {}", day, solver.part_one());
    println!("Day {} part two: {}", day, solver.part_two());
}
