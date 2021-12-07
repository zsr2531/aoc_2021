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

    fn part1(input: &(isize, Vec<isize>)) -> Self::Part1Out {
        let (mut pos, input) = input;
        let mut mid = calculate_fuel_part1(pos, &input);
        loop {
            let left = calculate_fuel_part1(pos - 1, &input);
            let right = calculate_fuel_part1(pos + 1, &input);

            if left < mid {
                mid = left;
                pos -= 1;
            } else if right < mid {
                mid = right;
                pos += 1;
            } else {
                return mid;
            }
        }
    }

    fn part2(input: &(isize, Vec<isize>)) -> Self::Part2Out {
        let (pos, input) = input;
        let mid = calculate_fuel_part2(*pos, &input);
        let left = calculate_fuel_part2(pos - 1, &input);
        let right = calculate_fuel_part2(pos + 1, &input);

        if left < mid {
            left
        } else if right < mid {
            right
        } else {
            mid
        }
    }
}

impl ParseInput<Day7> for AdventOfCode2021 {
    type Parsed = (isize, Vec<isize>);

    fn parse(input: &str) -> Self::Parsed {
        let mut sum = 0;
        let input: Vec<isize> = input
            .split(',')
            .map(|n| {
                let num = n.parse::<isize>().unwrap();
                sum += num;

                num
            })
            .collect();

        (sum / input.len() as isize, input)
    }
}
