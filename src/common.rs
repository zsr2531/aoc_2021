use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum Solution {
    Signed(i64),
    Unsigned(u64)
}

pub trait Solver {
    fn part1(&self, input: &str) -> Solution;
    fn part2(&self, input: &str) -> Solution;
}

impl Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Solution::Signed(x) => f.write_fmt(format_args!("{}", x)),
            Solution::Unsigned(x) => f.write_fmt(format_args!("{}", x))
        }
    }
}

impl From<i64> for Solution {
    fn from(x: i64) -> Self {
        Solution::Signed(x)
    }
}

impl From<u64> for Solution {
    fn from(x: u64) -> Self {
        Solution::Unsigned(x)
    }
}

impl From<usize> for Solution {
    fn from(x: usize) -> Self {
        Solution::Unsigned(x as u64)
    }
}
