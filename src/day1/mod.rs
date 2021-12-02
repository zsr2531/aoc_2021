use crate::common::*;

pub struct Day1;

impl Solver for Day1 {
    fn part1(&self, input: &str) -> Solution {
        let lines: Vec<i32> = input
            .lines()
            .map(|l| l.parse::<i32>().unwrap())
            .collect();

        lines
            .iter()
            .zip(lines.iter().skip(1))
            .filter(|(x, y)| x < y)
            .count()
            .into()
    }

    fn part2(&self, input: &str) -> Solution {
        let lines: Vec<i32> = input
            .lines()
            .map(|l| l.parse::<i32>().unwrap())
            .collect();
            
        lines
            .iter()
            .zip(lines.iter().skip(3))
            .filter(|(x, y)| x < y)
            .count()
            .into()
    }
}
