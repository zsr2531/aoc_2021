use super::common::*;

// couldn't figure this out on my own unfortunately
// kinda upset with myself, but oh well...
// https://github.com/ClxS/Advent-of-Code-2021/blob/main/AdventOfCode/Day06.cs
fn solve(days: usize, input: Vec<usize>) -> usize {
    let mut map = input.iter().fold([0_usize; 9], |mut count, curr| { count[*curr] += 1; count });
    (0..days).fold((0, input.len()), |(pivot, total), _| {
        let count = map[pivot];
        map[(pivot + 7) % 9] += count;

        ((pivot + 1) % 9, total + count)
    }).1
}

impl Solution<Day6> for AdventOfCode2021 {
    type Part1Out = usize;
    type Part2Out = usize;

    fn part1(input: Vec<usize>) -> Self::Part1Out {
        solve(80, input)
    }

    // ngl, part 2 got me :(
    fn part2(input: Vec<usize>) -> Self::Part2Out {
        solve(256, input)
    }
}

impl ParseInput<Day6> for AdventOfCode2021 {
    type Parsed = Vec<usize>;

    fn parse(input: &str) -> Self::Parsed {
        input
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect()
    }
}
