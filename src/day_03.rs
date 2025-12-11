use crate::aoc::Aoc;

#[derive(Debug)]
pub struct DayThree {
    input: String,
}

impl Aoc for DayThree {
    type PtOne = usize;
    type PtTwo = usize;

    fn new(input: String) -> Self {
        DayThree { input }
    }

    fn part_one(&mut self) -> Self::PtOne {
        self.max_voltages::<2>().sum()
    }

    fn part_two(&mut self) -> Self::PtTwo {
        self.max_voltages::<12>().sum()
    }
}

impl DayThree {
    fn battery_banks(&self) -> impl Iterator<Item = impl Iterator<Item = u8>> {
        self.input.trim().lines().map(|l| parse_digits(l))
    }

    fn max_voltages<const N: usize>(&self) -> impl Iterator<Item = usize> {
        self.battery_banks().map(|b| max_voltage::<N, _>(b))
    }
}

fn parse_digits(s: &str) -> impl Iterator<Item = u8> {
    s.chars().map(|c| (c as u8) - 48)
}

fn max_voltage<const N: usize, B>(mut bank: B) -> usize
where
    B: Iterator<Item = u8>,
{
    let mut digits = [0; N];

    for i in 0..N {
        digits[i] = bank.next().unwrap();
    }

    for x in bank {
        for i in 1..N {
            if digits[i] > digits[i - 1] {
                digits.copy_within(i.., i - 1);
                digits[N - 1] = x;
                break;
            }
        }

        if x > digits[N - 1] {
            digits[N - 1] = x;
        }
    }

    digits
        .into_iter()
        .enumerate()
        .map(|(i, x)| (x as usize) * 10usize.pow((N - i - 1) as u32))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
987654321111111
811111111111119
234234234234278
818181911112111
";

    #[test]
    fn test_part_one() {
        let banks: Vec<_> = DayThree::new(EXAMPLE.to_string())
            .max_voltages::<2>()
            .collect();

        assert_eq!(banks, vec![98, 89, 78, 92]);
    }

    #[test]
    fn test_part_two() {
        let banks: Vec<_> = DayThree::new(EXAMPLE.to_string())
            .max_voltages::<12>()
            .collect();

        assert_eq!(
            banks,
            vec![987654321111, 811111111119, 434234234278, 888911112111]
        );
    }
}
