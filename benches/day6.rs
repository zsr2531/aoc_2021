#![allow(dead_code)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[path ="../src/common.rs"]
mod common;

#[path ="../src/day6/mod.rs"]
mod day6;

use common::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../src/day6/input.txt");
    let parsed = <AdventOfCode2021 as ParseInput<Day6>>::parse(input);

    c.bench_function("day6 parse", |b| b.iter(|| <AdventOfCode2021 as ParseInput<Day6>>::parse(black_box(input))));
    c.bench_function("day6 part1", |b| b.iter(|| <AdventOfCode2021 as Solution<Day6>>::part1(black_box(&parsed))));
    c.bench_function("day6 part2", |b| b.iter(|| <AdventOfCode2021 as Solution<Day6>>::part2(black_box(&parsed))));
}

criterion_group!(day6, criterion_benchmark);
criterion_main!(day6);
