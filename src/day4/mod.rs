use super::common::*;

pub type DrawnNumbers = Vec<usize>;

#[derive(Clone, Copy)]
pub struct Board([[usize; 5]; 5]);

impl Solution<Day4> for AdventOfCode2021 {
    type Part1Out = usize;
    type Part2Out = usize;

    fn part1(input: &(DrawnNumbers, Vec<Board>)) -> Self::Part1Out {
        let (to_draw, boards) = input;
        let mut drawn = to_draw.iter().map(|n| *n).take(5).collect::<Vec<usize>>();

        while to_draw.len() > drawn.len() {
            if let Some(winner) = boards.iter().find(|b| b.is_winner(&drawn)) {
                let multiplier = drawn.last().unwrap();
                let sum = winner.sum_unmarked(&drawn);

                return sum * multiplier;
            }
            
            drawn.push(to_draw[drawn.len()]);
        }

        unreachable!("No solution found!");
    }

    fn part2(input: &(DrawnNumbers, Vec<Board>)) -> Self::Part2Out {
        let (mut drawn, boards) = (*input).clone();

        while let Some(last) = drawn.pop() {
            if let Some(loser) = boards.iter().find(|b| !b.is_winner(&drawn)) {
                drawn.push(last);
                let sum = loser.sum_unmarked(&drawn);

                return sum * last;
            }
        }

        unreachable!("No solution found!");
    }
}

impl ParseInput<Day4> for AdventOfCode2021 {
    type Parsed = (DrawnNumbers, Vec<Board>);

    fn parse(input: &str) -> Self::Parsed {
        let mut lines = input.lines();
        let mut boards = Vec::new();
        let drawn = lines
            .next()
            .unwrap()
            .split(',')
                .map(|x| x.parse::<usize>().unwrap())
            .collect();
        
        fn fill(dst: &mut [usize; 5], data: &str) {
            let mut parsed = data
                .split(' ')
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<usize>().unwrap());

            for i in 0..5 {
                dst[i] = parsed.next().unwrap();
            }
        }

        while let Some(_) = lines.next() {
            let mut buffer = [[0; 5]; 5];

            for i in 0..5 {
                fill(&mut buffer[i], lines.next().unwrap());
            }

            boards.push(Board(buffer));
        }

        (drawn, boards)
    }
}

impl Board {
    fn is_winner(&self, drawn: &[usize]) -> bool {
        // Check rows
        self.0.iter().any(|row| {
            row.iter().all(|n| drawn.contains(n))
        }) ||
        // Check columns
        (0..5).any(|col| {
            (0..5).all(|row| {
                drawn.contains(&self.0[row][col])
            })
        })
    }

    fn sum_unmarked(&self, drawn: &[usize]) -> usize {
        self.0.iter()
            .map(|row| row.iter().filter(|n| !drawn.contains(*n)).sum::<usize>())
            .sum()
    }
}
