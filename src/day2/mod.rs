use super::Solution;

pub struct Day2;

impl Solution for Day2 {
    fn part1(&self, input: &str) -> i64 {
        let (x, y) = input
            .lines()
            .fold((0, 0), |(dir, depth), l| {
                let mut s = l.split(' ');
                let (direction, distance) = (s.next(), s.next());
                let (direction, distance) = (direction.unwrap(), distance.unwrap().parse::<i32>().unwrap());

                match direction {
                    "forward" => (dir + distance, depth),
                    "up" => (dir, depth - distance),
                    "down" => (dir, depth + distance),
                    _ => unreachable!()
                }
            });

        (x * y) as i64
    }

    fn part2(&self, input: &str) -> i64 {
        let (x, y, _) = input
            .lines()
            .fold((0, 0, 0), |(dir, depth, aim), l| {
                let mut s = l.split(' ');
                let (direction, distance) = (s.next(), s.next());
                let (direction, distance) = (direction.unwrap(), distance.unwrap().parse::<i32>().unwrap());

                match direction {
                    "forward" => (dir + distance, depth + aim * distance, aim),
                    "up" => (dir, depth, aim - distance),
                    "down" => (dir, depth, aim + distance),
                    _ => unreachable!()
                }
            });

        (x * y) as i64
    }
}
