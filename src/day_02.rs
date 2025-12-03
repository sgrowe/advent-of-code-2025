use std::ops::RangeInclusive;

use crate::aoc::Aoc;

#[derive(Debug)]
pub struct DayTwo {
    ranges: Vec<RangeInclusive<usize>>,
}

impl Aoc for DayTwo {
    type PtOne = usize;
    type PtTwo = usize;

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

    fn part_one(&mut self) -> usize {
        self.invalid_ids_pt1().sum()
    }

    fn part_two(&mut self) -> usize {
        self.invalid_ids_pt2().sum()
    }
}

impl DayTwo {
    fn invalid_ids_pt1(&self) -> impl Iterator<Item = usize> {
        self.ranges
            .iter()
            .cloned()
            .flat_map(|r| r)
            .filter(|&x| is_invalid_pt1(x))
    }

    fn invalid_ids_pt2(&self) -> impl Iterator<Item = usize> {
        self.ranges
            .iter()
            .cloned()
            .flat_map(|r| r)
            .filter(|&x| is_invalid_pt2(x))
    }
}

fn is_invalid_pt1(id: usize) -> bool {
    let as_str = id.to_string();

    if as_str.len() % 2 != 0 {
        return false;
    }

    let mid = as_str.len() / 2;

    return &as_str[0..mid] == &as_str[mid..];
}

fn is_invalid_pt2(id: usize) -> bool {
    let as_str = id.to_string();

    for seq_len in 1..=as_str.len() / 2 {
        if as_str.len() % seq_len != 0 {
            continue;
        }

        let seq = &as_str[0..seq_len];

        let repeats = as_str.len() / seq_len;

        if (1..repeats)
            .map(|j| &as_str[(j * seq_len)..((j + 1) * seq_len)])
            .all(|next_seq| seq == next_seq)
        {
            return true;
        }
    }

    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn part_one_example() {
        let result: Vec<_> = DayTwo::new(EXAMPLE.to_string()).invalid_ids_pt1().collect();

        assert_eq!(
            result,
            vec![11, 22, 99, 1010, 1188511885, 222222, 446446, 38593859]
        )
    }

    #[test]
    fn part_two_example() {
        let result: Vec<_> = DayTwo::new(EXAMPLE.to_string()).invalid_ids_pt2().collect();

        assert_eq!(
            result,
            vec![
                11, 22, 99, 111, 999, 1010, 1188511885, 222222, 446446, 38593859, 565656,
                824824824, 2121212121
            ]
        )
    }
}
