use std::{io::{stdin, BufRead, Read}, env::args, fs::File, time::{Duration, Instant}};

mod common;
use common::*;

mod day1;
mod day2;
mod day3;
mod day4;

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
    ($year:ty, $day:ty, $input:expr) => {
        {
            let input1 = <$year as ParsePartInput<$day, 1>>::parse($input);
            let input2 = <$year as ParsePartInput<$day, 2>>::parse($input);
            let part1 = <$year as Solution<$day>>::part1(&input1);
            let part2 = <$year as Solution<$day>>::part2(&input2);

            (part1, part2)
        }
    };
}

macro_rules! benchmark {
    ($msg:literal, $code:expr) => {
        let mut sum = Duration::new(0, 0);

        for _ in 0..10000 {
            let start = Instant::now();
            $code
            sum += start.elapsed();
        }

        println!("{} took: {:?}", $msg, sum / 10000);
    };
    ($year:ty, $day:ty, $input:expr) => {
        benchmark!("Parse part 1", { <$year as ParsePartInput<$day, Part1>>::parse($input); });
        benchmark!("Parse part 2", { <$year as ParsePartInput<$day, Part2>>::parse($input); });
        let input1 = <$year as ParsePartInput<$day, Part1>>::parse($input);
        let input2 = <$year as ParsePartInput<$day, Part2>>::parse($input);
        benchmark!("Solving part 1", { <$year as Solution<$day>>::part1(&input1); });
        benchmark!("Solving part 2", { <$year as Solution<$day>>::part2(&input2); });
    };
}

macro_rules! run2021 {
    ($day:ty, $input:expr) => {
        {
            let (sol1, sol2) = run!(AdventOfCode2021, $day, $input);
            println!("Part 1: {}\nPart 2: {}", sol1, sol2);
            benchmark!(AdventOfCode2021, $day, $input);
        }
    };
}

fn run_day(day: usize, input: &str) {
    match day {
        1 => run2021!(Day1, input),
        2 => run2021!(Day2, input),
        3 => run2021!(Day3, input),
        _ => unreachable!()
    }
}

fn main() {
    let args = args().collect::<Vec<String>>();
    if args.len() != 2 {
        return eprintln!("Usage: {} <input1> <input2>", args[0]);
    }

    let input = read_input(&args[1]);
    let day = get_day(3);
    run_day(day, &input);
}
