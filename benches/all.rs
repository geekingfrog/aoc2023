use aoc2023::day04;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn separate(c: &mut Criterion) {
    c.bench_function("day04", |b| b.iter(|| black_box(day04::solve())));
}

criterion_group!(benches, separate);
criterion_main!(benches);
