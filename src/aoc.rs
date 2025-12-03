use std::fmt::Display;

pub trait Aoc {
    type PtOne: Display;
    type PtTwo: Display;

    fn new(input: String) -> Self;

    fn part_one(&mut self) -> Self::PtOne;
    fn part_two(&mut self) -> Self::PtTwo;
}
