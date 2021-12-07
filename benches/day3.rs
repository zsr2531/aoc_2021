#![allow(dead_code)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[path ="../src/common.rs"]
mod common;

#[path ="../src/day3/mod.rs"]
mod day3;

use common::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../src/day3/input.txt");
    let parsed = <AdventOfCode2021 as ParseInput<Day3>>::parse(input);

    c.bench_function("day3 parse", |b| b.iter(|| <AdventOfCode2021 as ParseInput<Day3>>::parse(black_box(input))));
    c.bench_function("day3 part1", |b| b.iter(|| <AdventOfCode2021 as Solution<Day3>>::part1(black_box(parsed.clone()))));
    c.bench_function("day3 part2", |b| b.iter(|| <AdventOfCode2021 as Solution<Day3>>::part2(black_box(parsed.clone()))));
}

criterion_group!(day3, criterion_benchmark);
criterion_main!(day3);
