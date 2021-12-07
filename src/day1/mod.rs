use crate::common::*;

impl Solution<Day1> for AdventOfCode2021 {
    type Part1Out = usize;
    type Part2Out = usize;

    fn part1(input: &Vec<i32>) -> Self::Part1Out {
        input
            .iter()
            .zip(input.iter().skip(1))
            .filter(|(x, y)| x < y)
            .count()
    }

    fn part2(input: &Vec<i32>) -> Self::Part2Out {
        input
            .iter()
            .zip(input.iter().skip(3))
            .filter(|(x, y)| x < y)
            .count()
    }
}

impl ParseInput<Day1> for AdventOfCode2021 {
    type Parsed = Vec<i32>;

    fn parse(input: &str) -> Self::Parsed {
        input
            .lines()
            .map(|l| l.parse::<i32>().unwrap())
            .collect()
    }
}
