use super::Solution;

pub struct Day1;

impl Solution for Day1 {
    fn part1(&self, input: &str) {
        let lines: Vec<i32> = input
            .lines()
            .map(|l| l.parse::<i32>().unwrap())
            .collect();

        let mut increments = 0;
        for i in 1..lines.len() {
            if lines[i - 1] < lines[i] {
                increments += 1;
            }
        }

        println!("{}", increments);
    }

    fn part2(&self, input: &str) {
        let lines: Vec<i32> = input
            .lines()
            .map(|l| l.parse::<i32>().unwrap())
            .collect();
            
        let mut windows: Vec<i32> = Vec::new();

        for i in 0..lines.len() - 2 {
            let mut buffer = [0; 3];

            for j in i..i + 3 {
                buffer[j - i] = lines[j];
            }

            windows.push(buffer.iter().sum());
        }

        let mut increments = 0;

        for i in 1..windows.len() {
            if windows[i - 1] < windows[i] {
                increments += 1;
            }
        }

        println!("{}", increments);
    }
}
