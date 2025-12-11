use std::collections::HashSet;

use crate::aoc::Aoc;

#[derive(Debug)]
pub struct DayFour {
    row_len: usize,
    col_len: usize,
    spots_with_rolls: HashSet<usize>,
}

type Coord = (usize, usize);

impl Aoc for DayFour {
    type PtOne = usize;
    type PtTwo = usize;

    fn new(input: String) -> Self {
        let mut total = 0;
        let mut col_len = 0;

        let spots: HashSet<_> = input
            .trim()
            .lines()
            .flat_map(|l| {
                col_len += 1;
                l.chars()
            })
            .enumerate()
            .filter_map(|(i, c)| {
                total = i + 1;

                if c == '@' { Some(i) } else { None }
            })
            .collect();

        let row_len = total / col_len;

        DayFour {
            row_len,
            col_len,
            spots_with_rolls: spots,
        }
    }

    fn part_one(&mut self) -> Self::PtOne {
        self.accessible_spots().count()
    }

    fn part_two(&mut self) -> Self::PtTwo {
        let mut spots: Vec<_> = self.accessible_spots().collect();
        let mut count = spots.len();

        while spots.len() > 0 {
            for x in spots {
                self.spots_with_rolls.remove(&x);
            }

            spots = self.accessible_spots().collect();
            count += spots.len();
        }

        count
    }
}

impl DayFour {
    fn accessible_spots(&self) -> impl Iterator<Item = usize> {
        self.spots_with_rolls.iter().copied().filter(|&c| {
            self.adjacent_to(c)
                .filter(|c| self.spots_with_rolls.contains(c))
                .count()
                < 4
        })
    }

    fn adjacent_to(&self, i: usize) -> impl Iterator<Item = usize> {
        let x = i % self.row_len;
        let y = i / self.row_len;

        let x_coords = [
            x.checked_sub(1),
            Some(x),
            Some(x + 1).filter(|&c| c < self.row_len),
        ];

        let y_coords = [
            y.checked_sub(1),
            Some(y),
            Some(y + 1).filter(|&c| c < self.col_len),
        ];

        x_coords
            .into_iter()
            .filter_map(|i| i)
            .flat_map(move |i| {
                y_coords
                    .into_iter()
                    .filter_map(|j| j)
                    .filter(move |&j| i != x || j != y)
                    .map(move |j| (i, j))
            })
            .map(|c| coord_to_index(c, self.row_len))
    }
}

fn coord_to_index((x, y): Coord, row_len: usize) -> usize {
    (y * row_len) + x
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

    #[test]
    fn test_part_one() {
        assert_eq!(DayFour::new(EXAMPLE.into()).part_one(), 13);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(DayFour::new(EXAMPLE.into()).part_two(), 43);
    }
}
