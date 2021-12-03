use crate::common::*;

pub struct Day3;

fn parse_binary_number(raw: &str) -> usize {
    let length = raw.len();

    raw
        .chars()
        .enumerate()
        .fold(0, |num, (idx, c)| {
            match c {
                '0' => num,
                '1' => num | (1 << (length - idx - 1)),
                _ => error(&format!("not a binary digit: {}", c))
            }
        })
}

impl Solver for Day3 {
    fn part1(&self, input: &str) -> Solution {
        let length = input.find(|c| { c == '\r' || c == '\n'}).unwrap();
        let mask = usize::MAX >> (64 - length);
        let numbers: Vec<usize> = input
            .lines()
            .map(|l| parse_binary_number(l))
            .collect();

        let gamma = (0..length)
            .fold(0_usize, |num, idx| {
                let ones = numbers
                    .iter()
                    .filter(|x| (*x & (1 << (length - idx - 1))) != 0)
                    .count();

                if ones >= (numbers.len() / 2) {
                    num | (1 << (length - idx - 1))
                } else {
                    num
                }
            });

        (gamma * (!gamma & mask)).into()
    }

    fn part2(&self, input: &str) -> Solution {
        todo!()
    }
}
