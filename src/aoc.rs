pub trait Aoc {
    fn new(input: String) -> Self;

    fn part_one(&mut self) -> isize;
    fn part_two(&mut self) -> isize;
}
