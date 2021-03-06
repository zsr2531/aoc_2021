#![allow(dead_code)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[path ="../src/common.rs"]
mod common;

#[path ="../src/day4/mod.rs"]
mod day4;

use common::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../src/day4/input.txt");
    let parsed = <AdventOfCode2021 as ParseInput<Day4>>::parse(input);

    c.bench_function("day4 parse", |b| b.iter(|| <AdventOfCode2021 as ParseInput<Day4>>::parse(black_box(input))));
    c.bench_function("day4 part1", |b| b.iter(|| <AdventOfCode2021 as Solution<Day4>>::part1(black_box(&parsed))));
    c.bench_function("day4 part2", |b| b.iter(|| <AdventOfCode2021 as Solution<Day4>>::part2(black_box(&parsed))));
}

criterion_group!(day4, criterion_benchmark);
criterion_main!(day4);
