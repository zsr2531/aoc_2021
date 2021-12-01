use std::{io::{stdin, BufRead, Read}, env::args, fs::File, time::Instant};

mod day1;

pub trait Solution {
    fn part1(&self, input: &str) -> i64;

    fn part2(&self, input: &str) -> i64;
}

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

macro_rules! benchmark {
    ($code:expr) => {
        let start = Instant::now();
        let result = $code;
        let end = Instant::now();

        println!("Solution: {}", result);
        println!("Took {:?}", end - start);
    };
}

fn main() {
    let args = args().skip(1).collect::<Vec<String>>();
    let solutions = vec![Box::new(day1::Day1)];
    let day = get_day(solutions.len()) - 1;
    let day = &solutions[day];

    match &args[..] {
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
        _ => eprintln!("Please provide the path to the input file via cmd args")
    }
}
