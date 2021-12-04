#![allow(non_upper_case_globals)]

use std::fmt::Display;

pub const Part1: u8 = 1;
pub const Part2: u8 = 2;

pub const Day1: u8 = 1;
pub const Day2: u8 = 2;
pub const Day3: u8 = 3;

pub struct AdventOfCode2021;

pub trait ParsePartInput<const Day: u8, const Part: u8> {
    type Parsed;

    fn parse(input: &str) -> Self::Parsed;
}

pub trait ParseInput<const Day: u8> {
    type Parsed;

    fn parse(input: &str) -> Self::Parsed;
}

pub trait Solution<const Day: u8>
where Self: ParsePartInput<Day, Part1> + ParsePartInput<Day, Part2> {
    type Part1Out: Display;
    type Part2Out: Display;

    fn part1(input: &<Self as ParsePartInput<Day, Part1>>::Parsed) -> Self::Part1Out;
    fn part2(input: &<Self as ParsePartInput<Day, Part2>>::Parsed) -> Self::Part2Out;
}

impl<T, const Day: u8> ParsePartInput<Day, Part1> for T
where T: ParseInput<Day> {
    type Parsed = T::Parsed;

    fn parse(input: &str) -> Self::Parsed {
        <Self as ParseInput<Day>>::parse(input)
    }
}

impl<T, const Day: u8> ParsePartInput<Day, Part2> for T
where T: ParseInput<Day> {
    type Parsed = T::Parsed;

    fn parse(input: &str) -> Self::Parsed {
        <Self as ParseInput<Day>>::parse(input)
    }
}
