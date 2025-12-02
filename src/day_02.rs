use std::ops::RangeInclusive;

use crate::aoc::Aoc;

#[derive(Debug)]
pub struct DayTwo {
    ranges: Vec<RangeInclusive<usize>>,
}

impl Aoc for DayTwo {
    fn new(input: String) -> Self {
        let ranges = input
            .trim()
            .split(',')
            .map(|s| {
                let (start, end) = s.split_once('-').unwrap();

                RangeInclusive::new(start.parse().unwrap(), end.parse().unwrap())
            })
            .collect();

        DayTwo { ranges }
    }

    fn part_one(&mut self) -> isize {
        self.invalid_ids().sum::<usize>() as isize
    }

    fn part_two(&mut self) -> isize {
        0
    }
}

impl DayTwo {
    fn invalid_ids(&self) -> impl Iterator<Item = usize> {
        self.ranges
            .iter()
            .cloned()
            .flat_map(|r| r)
            .filter(|&x| is_invalid(x))
    }
}

fn is_invalid(id: usize) -> bool {
    let as_str = id.to_string();

    // TODO:
    if as_str.len() % 2 != 0 {
        return false;
    }

    let mid = as_str.len() / 2;

    return &as_str[0..mid] == &as_str[mid..];
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn part_one_example() {
        let result: Vec<_> = DayTwo::new(EXAMPLE.to_string()).invalid_ids().collect();

        assert_eq!(
            result,
            vec![11, 22, 99, 1010, 1188511885, 222222, 446446, 38593859]
        )
    }
}
