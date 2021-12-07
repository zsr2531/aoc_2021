#![allow(dead_code)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[path ="../src/common.rs"]
mod common;

#[path ="../src/day2/mod.rs"]
mod day2;

use common::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../src/day2/input.txt");
    let parsed = <AdventOfCode2021 as ParseInput<Day2>>::parse(input);

    c.bench_function("day2 parse", |b| b.iter(|| <AdventOfCode2021 as ParseInput<Day2>>::parse(black_box(input))));
    c.bench_function("day2 part1", |b| b.iter(|| <AdventOfCode2021 as Solution<Day2>>::part1(black_box(parsed.clone()))));
    c.bench_function("day2 part2", |b| b.iter(|| <AdventOfCode2021 as Solution<Day2>>::part2(black_box(parsed.clone()))));
}

criterion_group!(day2, criterion_benchmark);
criterion_main!(day2);
