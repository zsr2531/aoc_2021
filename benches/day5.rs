#![allow(dead_code)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[path ="../src/common.rs"]
mod common;

#[path ="../src/day5/mod.rs"]
mod day5;

use common::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../src/day5/input.txt");
    let parsed = <AdventOfCode2021 as ParseInput<Day5>>::parse(input);

    c.bench_function("day5 parse", |b| b.iter(|| <AdventOfCode2021 as ParseInput<Day5>>::parse(black_box(input))));
    c.bench_function("day5 part1", |b| b.iter(|| <AdventOfCode2021 as Solution<Day5>>::part1(black_box(&parsed))));
    c.bench_function("day5 part2", |b| b.iter(|| <AdventOfCode2021 as Solution<Day5>>::part2(black_box(&parsed))));
}

criterion_group!(day5, criterion_benchmark);
criterion_main!(day5);
