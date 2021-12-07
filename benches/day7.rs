#![allow(dead_code)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[path ="../src/common.rs"]
mod common;

#[path ="../src/day7/mod.rs"]
mod day7;

use common::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../src/day7/input.txt");
    let parsed = <AdventOfCode2021 as ParseInput<Day7>>::parse(input);

    c.bench_function("day7 parse", |b| b.iter(|| <AdventOfCode2021 as ParseInput<Day7>>::parse(black_box(input))));
    c.bench_function("day7 part1", |b| b.iter(|| <AdventOfCode2021 as Solution<Day7>>::part1(black_box(parsed.clone()))));
    c.bench_function("day7 part2", |b| b.iter(|| <AdventOfCode2021 as Solution<Day7>>::part2(black_box(parsed.clone()))));
}

criterion_group!(day7, criterion_benchmark);
criterion_main!(day7);
