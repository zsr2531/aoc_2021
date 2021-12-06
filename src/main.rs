use std::{io::{stdin, BufRead, Read}, env::args, fs::File};

mod common;
use common::*;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn get_day(max: usize) -> usize {
    loop {
        println!("Select the day [1,{}]", max);
        let mut choice = String::new();
        stdin()
            .lock()
            .read_line(&mut choice)
            .expect("stdin unavailable");

        match choice[..1].parse::<usize>() {
            Ok(num) if num <= max => return num,
            _ => ()
        }
    }
}

fn read_input(path: &str) -> String {
    let mut buf = String::new();

    File::open(path)
        .expect("Failed to open file")
        .read_to_string(&mut buf)
        .expect("Failed to read file");

    buf
}

macro_rules! run {
    ($day:ty, $input:expr) => {
        {
            let input1 = <AdventOfCode2021 as ParsePartInput<$day, Part1>>::parse($input);
            let input2 = <AdventOfCode2021 as ParsePartInput<$day, Part2>>::parse($input);
            let part1 = <AdventOfCode2021 as Solution<$day>>::part1(&input1);
            let part2 = <AdventOfCode2021 as Solution<$day>>::part2(&input2);
            println!("Part 1: {}\nPart 2: {}", part1, part2);
        }
    };
}

fn run_day(day: usize, input: &str) {
    match day {
        1 => run!(Day1, input),
        2 => run!(Day2, input),
        3 => run!(Day3, input),
        4 => run!(Day4, input),
        5 => run!(Day5, input),
        6 => run!(Day6, input),
        _ => unreachable!()
    }
}

fn main() {
    let args = args().collect::<Vec<String>>();
    if args.len() != 2 {
        return eprintln!("Usage: {} <input1> <input2>", args[0]);
    }

    let input = read_input(&args[1]);
    let day = get_day(6);
    run_day(day, &input);
}
