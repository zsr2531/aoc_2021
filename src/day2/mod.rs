use crate::common::*;

#[derive(Debug, Clone, Copy)]
pub enum Command {
    Forward(isize),
    Up(isize),
    Down(isize)
}

impl Solution<Day2> for AdventOfCode2021 {
    type Part1Out = isize;
    type Part2Out = isize;

    fn part1(input: Vec<Command>) -> Self::Part1Out {
        let (hor, dep) = input.iter().fold((0, 0), |(hor, dep), c| {
            match c {
                Command::Forward(units) => (hor + units, dep),
                Command::Up(units) => (hor, dep - units),
                Command::Down(units) => (hor, dep + units),
            }
        });

        hor * dep
    }

    fn part2(input: Vec<Command>) -> Self::Part2Out {
        let (hor, dep, _) = input.iter().fold((0, 0, 0), |(hor, dep, aim), c| {
            match c {
                Command::Forward(units) => (hor + units, dep + aim * units, aim),
                Command::Up(units) => (hor, dep, aim - units),
                Command::Down(units) => (hor, dep, aim + units),
            }
        });

        hor * dep
    }
}

impl ParseInput<Day2> for AdventOfCode2021 {
    type Parsed = Vec<Command>;

    fn parse(input: &str) -> Self::Parsed {
        input
            .lines()
            .map(|l| {
                let bytes = l.as_bytes();
                let (dir, units) = (bytes[0] as char, bytes.last().unwrap());
                let units = (units - 48) as isize;
                match dir {
                    'f' => Command::Forward(units),
                    'u' => Command::Up(units),
                    'd' => Command::Down(units),
                    _ => unreachable!(&format!("Unknown command '{}'", l))
                }
            })
            .collect()
    }
}
