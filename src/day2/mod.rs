use super::Solution;

pub struct Day2;

impl Solution for Day2 {
    fn part1(&self, input: &str) -> i64 {
        let (x, y) = input
            .lines()
            .fold((0 as i64, 0 as i64), |(dir, depth), l| {
                let mut chars = l.chars();
                let (d, u) = (chars.nth(0).unwrap(), chars.nth_back(0).unwrap() as u8 - '0' as u8);
                let u = u as i64;
                match d {
                    'f' => (dir + u, depth),
                    'u' => (dir, depth - u),
                    'd' => (dir, depth + u),
                    _ => unreachable!()
                }
            });

        x * y
    }

    fn part2(&self, input: &str) -> i64 {
        let (x, y, _) = input
            .lines()
            .fold((0 as i64, 0 as i64, 0 as i64), |(dir, depth, aim), l| {
                let mut chars = l.chars();
                let (d, u) = (chars.nth(0).unwrap(), chars.nth_back(0).unwrap() as u8 - '0' as u8);
                let u = u as i64;
                match d {
                    'f' => (dir + u, depth + aim * u, aim),
                    'u' => (dir, depth, aim - u),
                    'd' => (dir, depth, aim + u),
                    _ => unreachable!()
                }
            });

        x * y
    }
}
