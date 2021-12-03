use crate::common::*;

pub struct Day3;

fn parse_binary_number(raw: &str) -> usize {
    let length = raw.len();

    raw
        .as_bytes()
        .iter()
        .enumerate()
        .fold(0, |num, (idx, c)| {
            match *c {
                48 => num,
                49 => num | (1 << (length - idx - 1)),
                _ => error(&format!("not a binary digit: {}", c))
            }
        })
}

impl Solver for Day3 {
    fn part1(&self, input: &str) -> Solution {
        let digits = input.find(|c| { c == '\r' || c == '\n'}).unwrap();
        let mask = usize::MAX >> (64 - digits);
        let numbers: Vec<usize> = input
            .lines()
            .map(|l| parse_binary_number(l))
            .collect();
        let threshold = numbers.len() / 2;

        let gamma = (0..digits)
            .fold(0_usize, |num, idx| {
                let ones = numbers
                    .iter()
                    .filter(|x| (*x & (1 << (digits - idx - 1))) != 0)
                    .count();

                if ones >= threshold {
                    num | (1 << (digits - idx - 1))
                } else {
                    num
                }
            });

        (gamma * (!gamma & mask)).into()
    }

    fn part2(&self, input: &str) -> Solution {
        let digits = input.find(|c| { c == '\r' || c == '\n'}).unwrap();
        let numbers: Vec<usize> = input
            .lines()
            .map(|l| parse_binary_number(l))
            .collect();

        let (mut oxygen, mut co2) = (numbers.clone(), numbers.clone());
        for idx in 0..digits {
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
            return (oxygen * co2).into();
        } else {
            error(&format!("More than 1 remaining oxygen or co2 values (oxygen: {:?}, co2: {:?})", oxygen, co2));
        }
    }
}
