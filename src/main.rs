use std::{fs::read_to_string, time::Instant};

use crate::{aoc::Aoc, day_01::DayOne, day_02::DayTwo, day_03::DayThree};

mod aoc;
mod day_01;
mod day_02;
mod day_03;

fn main() {
    solve::<DayOne>(1);
    solve::<DayTwo>(2);
    solve::<DayThree>(3);
}

fn solve<Day: Aoc>(day: usize) {
    let input = read_to_string(format!("inputs/day_{:02}.txt", day)).unwrap();

    let start = Instant::now();
    let mut solver = Day::new(input);

    let pt_1 = solver.part_one();
    let duration = start.elapsed();

    println!(
        "Day {} part one: {} - time taken: {:?}",
        day, pt_1, duration
    );

    let start = Instant::now();

    let pt_2 = solver.part_two();

    let duration = start.elapsed();
    println!(
        "Day {} part two: {} - time taken: {:?}",
        day, pt_2, duration
    );
}
