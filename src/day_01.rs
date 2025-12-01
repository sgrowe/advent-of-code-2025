use std::str::FromStr;

use crate::aoc::Aoc;

#[derive(Debug, Clone, Copy)]
struct Turn {
    dir: Dir,
    amount: isize,
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    Left,
    Right,
}

impl FromStr for Turn {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, amount) = s.split_at_checked(1).unwrap();

        let dir = match dir {
            "L" => Dir::Left,
            "R" => Dir::Right,
            _ => panic!("Unexpected dir {}", dir),
        };

        Ok(Turn {
            dir,
            amount: amount.parse()?,
        })
    }
}

#[derive(Debug)]
pub struct DayOne {
    instructions: Vec<Turn>,
}

impl DayOne {
    fn dial_states(&self) -> impl Iterator<Item = isize> {
        self.instructions.iter().scan(50, |dial, turn| {
            *dial = turn_dial(*dial, turn);

            Some(*dial)
        })
    }

    fn clicks(&self) -> impl Iterator<Item = isize> {
        self.instructions.iter().scan(50, |dial, turn| {
            let (new_dial, clicks) = turn_dial_2(*dial, turn);

            *dial = new_dial;

            Some(clicks)
        })
    }
}

impl Aoc for DayOne {
    fn new(input: String) -> Self {
        DayOne {
            instructions: input.trim().lines().map(|l| l.parse().unwrap()).collect(),
        }
    }

    fn part_one(&mut self) -> isize {
        self.dial_states().filter(|&dial| dial == 0).count() as isize
    }

    fn part_two(&mut self) -> isize {
        self.clicks().sum()
    }
}

fn turn_dial(dial: isize, Turn { dir, amount }: &Turn) -> isize {
    let dir = match dir {
        Dir::Left => -1,
        Dir::Right => 1,
    };

    let res = dial + (dir * (amount % 100));

    if res < 0 {
        res + 100
    } else if res > 99 {
        res - 100
    } else {
        res
    }
}

fn turn_dial_2(initial_dial: isize, Turn { dir, amount }: &Turn) -> (isize, isize) {
    let dir = match dir {
        Dir::Left => -1,
        Dir::Right => 1,
    };

    let mut clicks = amount / 100;

    let res = initial_dial + (dir * (amount % 100));

    if res == 0 {
        clicks += 1;
    }

    let dial = if res < 0 {
        if initial_dial > 0 {
            clicks += 1;
        }

        res + 100
    } else if res > 99 {
        clicks += 1;

        res - 100
    } else {
        res
    };

    (dial, clicks)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn part_one_example() {
        let result = DayOne::new(EXAMPLE.into()).part_one();

        assert_eq!(result, 3)
    }

    #[test]
    fn part_one_example_states() {
        let states: Vec<_> = DayOne::new(EXAMPLE.into()).dial_states().collect();

        assert_eq!(states, vec![82, 52, 0, 95, 55, 0, 99, 0, 14, 32]);
    }

    #[test]
    fn part_two_example() {
        let result = DayOne::new(EXAMPLE.into()).part_two();

        assert_eq!(result, 6)
    }

    #[test]
    fn part_two_example_states() {
        let states: Vec<_> = DayOne::new(EXAMPLE.into()).clicks().collect();

        assert_eq!(states, vec![1, 0, 1, 0, 1, 1, 0, 1, 0, 1]);
    }
}
