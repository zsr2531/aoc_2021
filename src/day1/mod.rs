use crate::common::*;

pub struct Day1;

fn solve(input: &str, offset: usize) -> usize {
    let lines: Vec<i32> = input
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .collect();
        
    lines
        .iter()
        .zip(lines.iter().skip(offset))
        .filter(|(x, y)| x < y)
        .count()
}

impl Solver for Day1 {
    fn part1(&self, input: &str) -> Solution {
        solve(input, 1).into()
    }

    fn part2(&self, input: &str) -> Solution {
        solve(input, 3).into()
    }
}
