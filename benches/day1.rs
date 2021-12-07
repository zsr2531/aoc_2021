#![allow(dead_code)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[path ="../src/common.rs"]
mod common;

#[path ="../src/day1/mod.rs"]
mod day1;

use common::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../src/day1/input.txt");
    let parsed = <AdventOfCode2021 as ParseInput<Day1>>::parse(input);

    c.bench_function("day1 parse", |b| b.iter(|| <AdventOfCode2021 as ParseInput<Day1>>::parse(black_box(input))));
    c.bench_function("day1 part1", |b| b.iter(|| <AdventOfCode2021 as Solution<Day1>>::part1(black_box(parsed.clone()))));
    c.bench_function("day1 part2", |b| b.iter(|| <AdventOfCode2021 as Solution<Day1>>::part2(black_box(parsed.clone()))));
}

criterion_group!(day1, criterion_benchmark);
criterion_main!(day1);
