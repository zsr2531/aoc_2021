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

macro_rules! solve {
    ($fuel:ident, $input: expr) => {
        {
            let (mut pos, input) = $input;
            let mut cost = $fuel(pos, &input);
            loop {
                let left = $fuel(pos - 1, &input);
                let right = $fuel(pos + 1, &input);

                if left < cost {
                    cost = left;
                    pos -= 1;
                } else if right < cost {
                    cost = right;
                    pos += 1;
                } else {
                    break;
                }
            }

            cost
        }
    };
}

impl Solution<Day7> for AdventOfCode2021 {
    type Part1Out = isize;
    type Part2Out = isize;

    fn part1(input: <Self as ParseInput<Day7>>::Parsed) -> Self::Part1Out {
        solve!(calculate_fuel_part1, input)
    }

    fn part2(input: <Self as ParseInput<Day7>>::Parsed) -> Self::Part2Out {
        solve!(calculate_fuel_part2, input)
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
