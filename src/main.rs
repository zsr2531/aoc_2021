use std::{io::{stdin, BufRead, Read}, env::args, fs::File};

mod day1;

pub trait Solution {
    fn part1(&self, input: &str);

    fn part2(&self, input: &str);
}

fn get_day(max: usize) -> usize {
    loop {
        println!("Select the day [1,{}]", max);
        let mut choice = String::new();
        stdin()
            .lock()
            .read_line(&mut choice)
            .expect("stdin unavailable");

        if let Ok(num) = choice[..1].parse::<usize>() {
            if num <= max {
                return num;
            }
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

fn main() {
    let args = args().skip(1).collect::<Vec<String>>();
    let solutions = vec![Box::new(day1::Day1)];
    let day = get_day(solutions.len()) - 1;
    let day = &solutions[day];

    println!("=============");

    if let [first, second] = &args[..] {
        let (part1, part2) = (read_input(first), read_input(second));

        println!("Part 1");
        day.part1(&part1);

        println!("=============");
        println!("Part 2");
        day.part2(&part2);
    } else if let [first] = &args[..] {
        let part1 = read_input(first);

        println!("Part 1");
        day.part1(&part1);
    } else {
        eprintln!("Please provide the path to the input file via cmd args");
    }
}
