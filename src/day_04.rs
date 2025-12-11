use crate::aoc::Aoc;

#[derive(Debug)]
pub struct DayFour {
    row_len: usize,
    grid: Vec<bool>,
}

impl Aoc for DayFour {
    type PtOne = usize;
    type PtTwo = usize;

    fn new(input: String) -> Self {
        let mut num_rows = 0;

        let grid: Vec<bool> = input
            .trim()
            .lines()
            .enumerate()
            .flat_map(|(i, l)| {
                num_rows = i + 1;
                l.chars().map(|c| c == '@')
            })
            .collect();

        DayFour {
            row_len: grid.len() / num_rows,
            grid,
        }
    }

    fn part_one(&mut self) -> Self::PtOne {
        self.accessible_spots().count()
    }

    fn part_two(&mut self) -> Self::PtTwo {
        let mut spots: Vec<_> = self.accessible_spots().collect();
        let mut count = spots.len();

        while spots.len() > 0 {
            for c in spots {
                *self.get_mut(c) = false;
            }

            spots = self.accessible_spots().collect();
            count += spots.len();
        }

        count
    }
}

type Coord = (usize, usize);

impl DayFour {
    fn accessible_spots(&self) -> impl Iterator<Item = Coord> {
        self.all_coords()
            .filter(|&c| self.get(c) && self.adjacent_to(c).filter(|&c| self.get(c)).count() < 4)
    }

    fn adjacent_to(&self, (x, y): Coord) -> impl Iterator<Item = Coord> {
        let x_coords = [
            x.checked_sub(1),
            Some(x),
            Some(x + 1).filter(|&c| c < self.row_len),
        ];

        let y_coords = [
            y.checked_sub(1),
            Some(y),
            Some(y + 1).filter(|&c| c < self.col_len()),
        ];

        x_coords.into_iter().filter_map(|i| i).flat_map(move |i| {
            y_coords
                .into_iter()
                .filter_map(|j| j)
                .filter(move |&j| i != x || j != y)
                .map(move |j| (i, j))
        })
    }

    fn get(&self, (x, y): Coord) -> bool {
        self.grid[y * self.col_len() + x]
    }

    fn get_mut(&mut self, (x, y): Coord) -> &mut bool {
        let i = y * self.col_len() + x;

        &mut self.grid[i]
    }

    fn col_len(&self) -> usize {
        self.grid.len() / self.row_len
    }

    fn all_coords(&self) -> impl Iterator<Item = Coord> {
        (0..self.col_len()).flat_map(move |c| (0..self.row_len).map(move |r| (r, c)))
    }
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
