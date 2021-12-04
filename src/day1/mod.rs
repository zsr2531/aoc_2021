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

// pub struct Day1;

// fn solve(input: &str, offset: usize) -> usize {
//     let lines: Vec<i32> = input
//         .lines()
//         .map(|l| l.parse::<i32>().unwrap())
//         .collect();
        
//     lines
//         .iter()
//         .zip(lines.iter().skip(offset))
//         .filter(|(x, y)| x < y)
//         .count()
// }

// impl Solver for Day1 {
//     fn part1(&self, input: &str) -> Solution {
//         solve(input, 1).into()
//     }

//     fn part2(&self, input: &str) -> Solution {
//         solve(input, 3).into()
//     }
// }
