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
        self.max_voltages().sum()
    }

    fn part_two(&mut self) -> Self::PtTwo {
        0
    }
}

impl DayThree {
    fn battery_banks(&self) -> impl Iterator<Item = impl Iterator<Item = u8>> {
        self.input.trim().lines().map(|l| parse_digits(l))
    }

    fn max_voltages(&self) -> impl Iterator<Item = usize> {
        self.battery_banks().map(|b| max_voltage(b))
    }
}

fn parse_digits(s: &str) -> impl Iterator<Item = u8> {
    s.chars().map(|c| (c as u8) - 48)
}

fn max_voltage<B>(mut bank: B) -> usize
where
    B: Iterator<Item = u8>,
{
    let mut a = bank.next().unwrap();
    let mut b = bank.next().unwrap();

    for x in bank {
        if b > a {
            a = b;
            b = x;
        } else if x > b {
            b = x;
        } else if x > a {
            a = x;
        }
    }

    ((a * 10) + b) as usize
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
    fn test_max_voltage() {
        let banks: Vec<_> = DayThree::new(EXAMPLE.to_string()).max_voltages().collect();

        assert_eq!(banks, vec![98, 89, 78, 92,]);
    }
}
