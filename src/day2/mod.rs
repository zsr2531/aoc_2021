use crate::common::*;

pub struct Day2;

impl Solver for Day2 {
    fn part1(&self, input: &str) -> Solution {
        let (x, y) = input
            .lines()
            .fold((0 as u64, 0 as u64), |(hor, dep), l| {
                let mut chars = l.chars();
                let (dir, units) = (chars.nth(0).unwrap(), chars.nth_back(0).unwrap() as u8 - '0' as u8);
                let units = units as u64;
                match dir {
                    'f' => (hor + units, dep),
                    'u' => (hor, dep - units),
                    'd' => (hor, dep + units),
                    _ => error(&format!("Unknown command '{}'", l))
                }
            });

        (x * y).into()
    }

    fn part2(&self, input: &str) -> Solution {
        let (x, y, _) = input
            .lines()
            .fold((0 as u64, 0 as u64, 0 as u64), |(hor, dep, aim), l| {
                let mut chars = l.chars();
                let (dir, units) = (chars.nth(0).unwrap(), chars.nth_back(0).unwrap() as u8 - '0' as u8);
                let units = units as u64;
                match dir {
                    'f' => (hor + units, dep + aim * units, aim),
                    'u' => (hor, dep, aim - units),
                    'd' => (hor, dep, aim + units),
                    _ => error(&format!("Unknown command '{}'", l))
                }
            });

        (x * y).into()
    }
}
