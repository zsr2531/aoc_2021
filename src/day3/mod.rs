use crate::common::*;

impl Solution<Day3> for AdventOfCode2021 {
    type Part1Out = usize;
    type Part2Out = usize;

    fn part1(input: &(usize, Vec<usize>)) -> Self::Part1Out {
        let (digits, numbers) = input;
        let mask = usize::MAX >> (64 - digits);
        let threshold = numbers.len() / 2;

        let gamma = (0..*digits)
            .fold(0, |num, idx| {
                let ones = numbers
                    .iter()
                    .filter(|x| (*x & (1 << (digits - idx - 1))) != 0)
                    .count();

                num * 2 + usize::from(ones >= threshold)
            });

        gamma * (!gamma & mask)
    }

    fn part2(input: &(usize, Vec<usize>)) -> Self::Part2Out {
        let (digits, numbers) = input;
        let (mut oxygen, mut co2) = (numbers.clone(), numbers.clone());
        for idx in 0..*digits {
            if oxygen.len() > 1 {
                let ones = oxygen
                    .iter()
                    .filter(|x| (*x & (1 << (digits - idx - 1))) != 0)
                    .count();
                let zeros = oxygen.len() - ones;

                if ones >= zeros {
                    oxygen.retain(|x| (x & (1 << (digits - idx - 1))) != 0);
                } else if ones < zeros {
                    oxygen.retain(|x| (x & (1 << (digits - idx - 1))) == 0);
                }
            }

            if co2.len() > 1 {
                let ones = co2
                    .iter()
                    .filter(|x| (*x & (1 << (digits - idx - 1))) != 0)
                    .count();
                let zeros = co2.len() - ones;

                if ones >= zeros {
                    co2.retain(|x| (x & (1 << (digits - idx - 1))) == 0);
                } else if ones < zeros {
                    co2.retain(|x| (x & (1 << (digits - idx - 1))) != 0);
                }
            }
        }

        if let ([oxygen], [co2]) = (&oxygen[..], &co2[..]) {
            oxygen * co2
        } else {
            unreachable!(&format!("More than 1 remaining oxygen or co2 values (oxygen: {:?}, co2: {:?})", oxygen, co2));
        }
    }
}

impl ParseInput<Day3> for AdventOfCode2021 {
    type Parsed = (usize, Vec<usize>);

    fn parse(input: &str) -> Self::Parsed {
        (input.find(|c| { c == '\r' || c == '\n' }).unwrap(), input
            .lines()
            .map(|l| usize::from_str_radix(l, 2).unwrap())
            .collect())
    }
}
