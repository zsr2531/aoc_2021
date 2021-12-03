use std::{io::{stdin, BufRead, Read}, env::args, fs::File, time::{Duration, Instant}};

mod common;
use common::Solver;

mod day1;
mod day2;
mod day3;

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

macro_rules! pack {
    ($($code:expr),+) => {
        vec![$(Box::new($code)),+]
    };
}

macro_rules! benchmark {
    ($code:expr) => {
        let result = $code;
        let mut sum = Duration::new(0, 0);

        for _ in 0..1000 {
            let start = Instant::now();
            $code;
            let end = Instant::now();
            sum += (end - start);
        }

        println!("Solution: {}", result);
        println!("Time: {:?}", sum / 1000);
    };
}

fn main() {
    let args = args().collect::<Vec<String>>();
    if args.len() < 2 {
        return eprintln!("Usage: {} <input1> <input2>", args[0]);
    }

    let solutions: Vec<Box<dyn Solver>> = pack![day1::Day1, day2::Day2, day3::Day3];
    let day = get_day(solutions.len()) - 1;
    let day = &solutions[day];

    match &args[1..] {
        [first] => {
            let part1 = read_input(first);

            println!("==== PART 1 ====");
            benchmark!(day.part1(&part1));
        }
        [first, second] => {
            let (part1, part2) = (read_input(first), read_input(second));

            println!("==== PART 1 ====");
            benchmark!(day.part1(&part1));

            println!("==== PART 2 ====");
            benchmark!(day.part2(&part2));
        }
        _ => eprintln!("Usage: {} <input1> <input2>", args[0])
    }
}
