use super::common::*;

fn calculate_fuel_part1(pos: isize, crabs: &Vec<isize>) -> isize {
    crabs
        .iter()
        .map(|p| (*p - pos).abs())
        .sum()
}

fn calculate_fuel_part2(pos: isize, crabs: &Vec<isize>) -> isize {
    crabs
        .iter()
        .map(|p| {
            let n = (*p - pos).abs();
            n * (n + 1) / 2
        })
        .sum()
}

impl Solution<Day7> for AdventOfCode2021 {
    type Part1Out = isize;
    type Part2Out = isize;

    fn part1(input: &Vec<isize>) -> Self::Part1Out {
        let mut input = input.clone();
        input.sort();
        let pos = input[input.len() / 2];
        calculate_fuel_part1(pos, &input)
    }

    fn part2(input: &Vec<isize>) -> Self::Part2Out {
        let pos = input.iter().sum::<isize>() / input.len() as isize;

        let mid = calculate_fuel_part2(pos, &input);
        // let left = calculate_fuel_part2(pos - 1, &input);
        let right = calculate_fuel_part2(pos + 1, &input);

        // if left < mid {
        //     left
        if right < mid {
            right
        } else {
            mid
        }
    }
}

impl ParseInput<Day7> for AdventOfCode2021 {
    type Parsed = Vec<isize>;

    fn parse(input: &str) -> Self::Parsed {
        input
            .split(',')
            .map(|n| n.parse::<isize>().unwrap())
            .collect()
    }
}
