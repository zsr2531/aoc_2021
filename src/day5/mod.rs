use super::common::*;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Point(isize, isize);

#[derive(Debug, Clone, Copy)]
pub struct Segment(Point, Point);

pub struct Grid {
    pub count: usize,
    data: Vec<u8>,
}

impl Solution<Day5> for AdventOfCode2021 {
    type Part1Out = usize;
    type Part2Out = usize;

    fn part1(input: Vec<Segment>) -> Self::Part1Out {
        let mut grid = Grid::new();

        input
            .iter()
            .filter(|s| s.0.0 == s.1.0 || s.0.1 == s.1.1)
            .for_each(|s| {
                let (x1, y1) = (s.0.0, s.0.1);
                let (x2, y2) = (s.1.0, s.1.1);

                if x1 == x2 {
                    for i in y1.min(y2)..=y1.max(y2) {
                        let point = Point(x1, i);
                        grid.increment(&point);
                    }
                } else {
                    for i in x1.min(x2)..=x1.max(x2) {
                        let point = Point(i, y1);
                        grid.increment(&point);
                    }
                }
            });
        

        grid.count
    }

    fn part2(input: Vec<Segment>) -> Self::Part2Out {
        let mut grid = Grid::new();

        input
            .iter()
            .for_each(|s| {
                let (mut x1, mut y1) = (s.0.0, s.0.1);
                let (x2, y2) = (s.1.0, s.1.1);

                fn get_diff(a: isize, b: isize) -> isize {
                    use std::cmp::Ordering;
                    match a.cmp(&b) {
                        Ordering::Less => 1,
                        Ordering::Equal => 0,
                        Ordering::Greater => -1,
                    }
                }

                if x1 == x2 {
                    for i in y1.min(y2)..=y1.max(y2) {
                        let point = Point(x1, i);
                        grid.increment(&point);
                    }
                } else if y1 == y2 {
                    for i in x1.min(x2)..=x1.max(x2) {
                        let point = Point(i, y1);
                        grid.increment(&point);
                    }
                } else {
                    let (x_off, y_off) = (get_diff(x1, x2), get_diff(y1, y2));
                    
                    loop {
                        let point = Point(x1, y1);
                        grid.increment(&point);

                        if x1 == x2 || y1 == y2 {
                            break;
                        }

                        x1 += x_off;
                        y1 += y_off;
                    }
                }
            });
        

        grid.count
    }
}

impl ParseInput<Day5> for AdventOfCode2021 {
    type Parsed = Vec<Segment>;

    fn parse(input: &str) -> Self::Parsed {
        input
            .lines()
            .map(|l| {
                let (s1, s2) = l.split_once(" -> ").unwrap();
                let (x1, y1) = s1.split_once(',').map(|(x, y)| (x.parse::<isize>().unwrap(), y.parse::<isize>().unwrap())).unwrap();
                let (x2, y2) = s2.split_once(',').map(|(x, y)| (x.parse::<isize>().unwrap(), y.parse::<isize>().unwrap())).unwrap();

                Segment(Point(x1, y1), Point(x2, y2))
            })
            .collect()
    }
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            count: 0,
            data: vec![0; 999 * 999]
        }
    }

    pub fn increment(&mut self, at: &Point) {
        let index = (at.0 * 999 + at.1) as usize;
        self.data[index] += 1;

        if self.data[index] == 2 {
            self.count += 1;
        }
    }
}
