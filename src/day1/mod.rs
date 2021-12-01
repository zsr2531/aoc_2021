use super::Solution;

pub struct Day1;

impl Solution for Day1 {
    fn part1(&self, input: &str) -> i64 {
        let lines: Vec<i32> = input
            .lines()
            .map(|l| l.parse::<i32>().unwrap())
            .collect();

        lines
            .iter()
            .zip(lines.iter().skip(1))
            .filter(|(x, y)| x < y)
            .count() as i64
    }

    fn part2(&self, input: &str) -> i64 {
        let lines: Vec<i32> = input
            .lines()
            .map(|l| l.parse::<i32>().unwrap())
            .collect();
            
        lines
            .iter()
            .zip(lines.iter().skip(3))
            .filter(|(x, y)| x < y)
            .count() as i64
    }
}
