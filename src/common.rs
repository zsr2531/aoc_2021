use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Copy)]
pub enum Solution {
    Signed(i64),
    Unsigned(u64)
}

pub trait Solver {
    fn part1(&self, input: &str) -> Solution;
    fn part2(&self, input: &str) -> Solution;
}

pub fn error(msg: &str) -> ! {
    panic!("Something went wrong (maybe you loaded the wrong day?): {}", msg)
}

impl Display for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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
